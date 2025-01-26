use crate::error::Error;
use crate::repos::{WhoisPerson, WhoisRepository};
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

const GET_PERSON: &str = "SELECT * FROM whois WHERE discord_id= $1";
const ADD_PERSON: &str = "INSERT INTO whois(discord_id, human_name, hash_name) VALUES( $1, $2, $3)";
const UPDATE_PERSON: &str = "UPDATE whois SET human_name= $1, hash_name= $2 WHERE discord_id= $3";

#[derive(Clone, Debug)]
pub struct PostgresWhoisRepository {
    pool: Pool<Postgres>,
}

impl PostgresWhoisRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl WhoisRepository for PostgresWhoisRepository {
    async fn get_by_discord_id(&self, discord_id: u64) -> Result<Option<WhoisPerson>, Error> {
        let person = sqlx::query(GET_PERSON)
            .bind(discord_id as i64)
            .fetch_one(&self.pool)
            .await
            .ok()
            .map(row_to_person);
        Ok(person)
    }

    async fn add(&self, discord_id: u64, human_name: &str, hash_name: &str) -> Result<(), Error> {
        sqlx::query(ADD_PERSON)
            .bind(discord_id as i64)
            .bind(human_name)
            .bind(hash_name)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;
        Ok(())
    }

    async fn update(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> Result<(), Error> {
        sqlx::query(UPDATE_PERSON)
            .bind(human_name)
            .bind(hash_name)
            .bind(discord_id as i64)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}

fn row_to_person(row: PgRow) -> WhoisPerson {
    let discord_id: i64 = row.get("discord_id");
    let human_name: String = row.get("human_name");
    let hash_name: String = row.get("hash_name");
    WhoisPerson {
        discord_id: discord_id as u64,
        human_name,
        hash_name,
    }
}

#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_repo() {
        let time = chrono::Utc::now().timestamp_millis();
        let user_id = time as u64;
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect("postgresql://pog_user:pog_pass@localhost:5432/pog_server")
            .await
            .expect("unable to connect to database");
        let repo = PostgresWhoisRepository::new(pool);
        repo.add(user_id, "test", "test").await.unwrap();
        let person = repo.get_by_discord_id(user_id).await.unwrap();
        assert_eq!(
            person,
            Some(WhoisPerson {
                discord_id: user_id,
                human_name: "test".to_string(),
                hash_name: "test".to_string(),
            })
        );
        repo.update(user_id, "test2", "test2").await.unwrap();
        let person = repo.get_by_discord_id(user_id).await.unwrap();
        assert_eq!(
            person,
            Some(WhoisPerson {
                discord_id: user_id,
                human_name: "test2".to_string(),
                hash_name: "test2".to_string(),
            })
        );
    }
}
