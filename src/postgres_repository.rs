use crate::error::Error;
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;
use futures::TryStreamExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row, Transaction};

const INSERT_WAGER: &str = "INSERT INTO wagers(wager_id,time,offering,accepting,wager,outcome,status) VALUES (nextval('seq_wager_id'), $1, $2, $3, $4, $5, $6)";
const SELECT_BY_USER: &str =
    "SELECT * FROM wagers WHERE (offering= $1 OR accepting= $2) AND status=0";

#[derive(Clone, Debug)]
pub struct PostgresWagerRepo {
    pool: Pool<Postgres>,
}

impl PostgresWagerRepo {
    pub async fn new(connection_string: &str) -> PostgresWagerRepo {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(connection_string)
            .await
            .expect("unable to connect to database");
        PostgresWagerRepo { pool }
    }
}

#[async_trait::async_trait]
impl WagerRepository for PostgresWagerRepo {
    async fn insert(&self, wager: Wager) -> Result<(), Error> {
        let mut tx: Transaction<Postgres> = sqlx::Acquire::begin(&self.pool).await?;
        let result = sqlx::query(INSERT_WAGER)
            .bind(wager.time)
            .bind(wager.offering)
            .bind(wager.accepting)
            .bind(wager.wager)
            .bind(wager.outcome)
            .bind(wager.status.as_i16())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        match result.rows_affected() {
            1 => Ok(()),
            num => {
                let msg = format!("attempt to insert wager submitted {} lines", num);
                Err(Error::DatabaseFailure(msg))
            }
        }
    }

    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error> {
        let mut rows = sqlx::query(SELECT_BY_USER)
            .bind(user)
            .bind(user)
            .fetch(&self.pool);
        let mut result: Vec<Wager> = Default::default();
        while let Some(row) = rows.try_next().await.map_err(Error::from)? {
            let time: String = row.get("time");
            let offering: String = row.get("offering");
            let accepting: String = row.get("accepting");
            let wager: String = row.get("wager");
            let outcome: String = row.get("outcome");
            let status: i16 = row.get("status");
            let wager = Wager {
                time,
                offering,
                accepting,
                wager,
                outcome,
                status: WagerStatus::from_i16(status),
            };
            result.push(wager);
        }
        Ok(result)
    }
}

#[tokio::test]
async fn test_repo() {
    let repo = PostgresWagerRepo::new("postgresql://pog_user:pog_pass@127.0.0.1:5432/pog").await;
    let user_a = uuid::Uuid::new_v4().to_string();
    let user_b = uuid::Uuid::new_v4().to_string();
    let user_c = uuid::Uuid::new_v4().to_string();
    let time = chrono::Utc::now().to_rfc3339();
    repo.insert(Wager {
        time: time.to_string(),
        offering: user_a.to_string(),
        accepting: user_b.to_string(),
        wager: "$100".to_string(),
        outcome: "Rangers take the Phillies, should they meet".to_string(),
        status: WagerStatus::Open,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        time: time.to_string(),
        offering: user_c.to_string(),
        accepting: user_a.to_string(),
        wager: "$40".to_string(),
        outcome: "Jax has a losing season".to_string(),
        status: WagerStatus::Open,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        time: time.to_string(),
        offering: user_b.to_string(),
        accepting: user_c.to_string(),
        wager: "$30".to_string(),
        outcome: "Jets beat the Oilers".to_string(),
        status: WagerStatus::Open,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        time: time.to_string(),
        offering: user_b.to_string(),
        accepting: user_c.to_string(),
        wager: "$30".to_string(),
        outcome: "Something that already happened".to_string(),
        status: WagerStatus::Paid,
    })
        .await
        .unwrap();
    let found = repo.search_by_user(&user_b).await.unwrap();
    assert_eq!(2, found.len());
}