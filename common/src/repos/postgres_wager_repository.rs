use crate::discord_id::DiscordId;
use crate::error::Error;
use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row, Transaction};

const INSERT_WAGER: &str = r#"INSERT INTO wagers(wager_id,time,offering,resolved_offering_user,accepting,resolved_accepting_user,wager,outcome,status,expected_settle_date)
        VALUES (nextval('seq_wager_id'), $1, $2, $3, $4, $5, $6, $7, $8, $9)"#;
const SELECT_BY_ID: &str = "SELECT * FROM wagers WHERE wager_id= $1";
const SELECT_BY_USER: &str =
    "SELECT * FROM wagers WHERE (offering= $1 OR accepting= $2) AND status=0";
const SELECT_BY_USER_ID: &str =
    "SELECT * FROM wagers WHERE (resolved_offering_user= $1 OR resolved_accepting_user= $2) AND status=0";
const UPDATE_STATUS: &str = "UPDATE wagers SET status= $1 WHERE wager_id= $2";

#[derive(Clone, Debug)]
pub struct PostgresWagerRepo {
    pool: Pool<Postgres>,
}

impl PostgresWagerRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

use crate::repos::WagerRepository;
use crate::wager::{Wager, WagerStatus};

impl WagerRepository for PostgresWagerRepo {
    async fn insert(&self, wager: Wager) -> Result<(), Error> {
        let resolved_offering_user: Option<i64> = wager.resolved_offering_user.map(|v| v.value());
        let resolved_accepting_user: Option<i64> = wager.resolved_accepting_user.map(|v| v.value());
        let mut tx: Transaction<Postgres> = sqlx::Acquire::begin(&self.pool).await?;
        let result = sqlx::query(INSERT_WAGER)
            .bind(wager.time)
            .bind(wager.offering)
            .bind(resolved_offering_user)
            .bind(wager.accepting)
            .bind(resolved_accepting_user)
            .bind(wager.wager)
            .bind(wager.outcome)
            .bind(wager.status.as_i16())
            .bind(wager.expected_settle_date.map(|s| s.to_string()))
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        match result.rows_affected() {
            1 => Ok(()),
            num => {
                let msg = format!("attempt to insert wager submitted {} lines", num);
                Err(Error::Database(msg))
            }
        }
    }

    async fn get(&self, wager_id: i32) -> Option<Wager> {
        sqlx::query(SELECT_BY_ID)
            .bind(wager_id)
            .fetch_one(&self.pool)
            .await
            .ok()
            .map(row_to_wager)
    }

    async fn search_by_user_id(&self, user_id: &DiscordId) -> Result<Vec<Wager>, Error> {
        let mut rows = sqlx::query(SELECT_BY_USER_ID)
            .bind(user_id.value())
            .bind(user_id.value())
            .fetch(&self.pool);
        let mut result: Vec<Wager> = Default::default();
        while let Some(row) = rows.try_next().await.map_err(Error::from)? {
            result.push(row_to_wager(row));
        }
        Ok(result)
    }

    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error> {
        let mut rows = sqlx::query(SELECT_BY_USER)
            .bind(user)
            .bind(user)
            .fetch(&self.pool);
        let mut result: Vec<Wager> = Default::default();
        while let Some(row) = rows.try_next().await.map_err(Error::from)? {
            result.push(row_to_wager(row));
        }
        Ok(result)
    }

    async fn update_status(&self, wager_id: i32, wager: &Wager) -> Result<(), Error> {
        let status = wager.status.as_i16();
        sqlx::query(UPDATE_STATUS)
            .bind(status)
            .bind(wager_id)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}

fn row_to_wager(row: PgRow) -> Wager {
    let time: String = row.get("time");
    let wager_id: i32 = row.get("wager_id");
    let offering: String = row.get("offering");
    let resolved_offering_user_id: Option<i64> = row.get("resolved_offering_user");
    let resolved_offering_user = resolved_offering_user_id.map(Into::into);
    let accepting: String = row.get("accepting");
    let resolved_accepting_user_id: Option<i64> = row.get("resolved_accepting_user");
    let resolved_accepting_user = resolved_accepting_user_id.map(Into::into);
    let wager: String = row.get("wager");
    let outcome: String = row.get("outcome");
    let status: i16 = row.get("status");
    let expected_settle_date_str: Option<String> = row.get("expected_settle_date");
    let expected_settle_date = expected_settle_date_str.map(|s| s.parse().unwrap());
    Wager {
        wager_id: wager_id as u32,
        time,
        offering,
        resolved_offering_user,
        accepting,
        resolved_accepting_user,
        wager,
        outcome,
        status: WagerStatus::from_i16(status),
        expected_settle_date,
    }
}

#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod test {
    use chrono::NaiveDate;

    use crate::repos::{new_db_pool, PostgresWagerRepo, WagerRepository};
    use crate::wager::{Wager, WagerStatus};

    #[tokio::test]
    async fn repo() {
        let db_pool = new_db_pool("postgresql://pog_user:pog_pass@127.0.0.1:5432/pog_server").await;
        let repo = PostgresWagerRepo::new(db_pool.clone());
        let user_a = uuid::Uuid::new_v4().to_string();
        let user_b = uuid::Uuid::new_v4().to_string();
        let user_c = uuid::Uuid::new_v4().to_string();
        let time = chrono::Utc::now().to_rfc3339();
        repo.insert(Wager {
            wager_id: 0,
            time: time.to_string(),
            offering: user_a.to_string(),
            resolved_offering_user: None,
            accepting: user_b.to_string(),
            resolved_accepting_user: None,
            wager: "$100".to_string(),
            outcome: "Rangers take the Phillies, should they meet".to_string(),
            status: WagerStatus::Open,
            expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
        })
        .await
        .unwrap();
        repo.insert(Wager {
            wager_id: 0,
            time: time.to_string(),
            offering: user_c.to_string(),
            resolved_offering_user: None,
            accepting: user_a.to_string(),
            resolved_accepting_user: None,
            wager: "$40".to_string(),
            outcome: "Jax has a losing season".to_string(),
            status: WagerStatus::Open,
            expected_settle_date: None,
        })
        .await
        .unwrap();
        repo.insert(Wager {
            wager_id: 0,
            time: time.to_string(),
            offering: user_b.to_string(),
            resolved_offering_user: None,
            accepting: user_c.to_string(),
            resolved_accepting_user: None,
            wager: "$30".to_string(),
            outcome: "Jets beat the Oilers".to_string(),
            status: WagerStatus::Open,
            expected_settle_date: None,
        })
        .await
        .unwrap();
        repo.insert(Wager {
            wager_id: 0,
            time: time.to_string(),
            offering: user_b.to_string(),
            resolved_offering_user: None,
            accepting: user_c.to_string(),
            resolved_accepting_user: None,
            wager: "$30".to_string(),
            outcome: "Something that already happened".to_string(),
            status: WagerStatus::Paid,
            expected_settle_date: None,
        })
        .await
        .unwrap();
        let found = repo.search_by_user(&user_b).await.unwrap();
        assert_eq!(2, found.len());
    }
}
