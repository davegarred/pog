use crate::error::Error;
use crate::repos::{AdminRepository, AdminSettings};
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

const DEFAULT_SETTINGS_ASSIGNMENT: &str = "default";
const GET_ADMIN: &str = "SELECT settings FROM admin_settings WHERE assignment= $1";
const UPDATE_ADMIN: &str = "UPDATE admin_settings SET settings= $1 WHERE assignment= $2";

#[derive(Clone, Debug)]
pub struct PostgresAdminRepository {
    pool: Pool<Postgres>,
}

impl PostgresAdminRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl AdminRepository for PostgresAdminRepository {
    async fn get(&self) -> Result<AdminSettings, Error> {
        match sqlx::query(GET_ADMIN)
            .bind(DEFAULT_SETTINGS_ASSIGNMENT)
            .fetch_one(&self.pool)
            .await
            .ok()
            .map(row_to_settings)
        {
            Some(admin) => Ok(admin),
            None => Err(Error::Unexpected("no admin settings found".to_string())),
        }
    }

    async fn update(&self, settings: AdminSettings) -> Result<(), Error> {
        let val = serde_json::to_value(&settings).expect("unable to deserialize admin settings");
        sqlx::query(UPDATE_ADMIN)
            .bind(val)
            .bind(DEFAULT_SETTINGS_ASSIGNMENT)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}

fn row_to_settings(row: PgRow) -> AdminSettings {
    let settings: Value = row.get("settings");
    serde_json::from_value(settings).expect("unable to deserialize admin settings from json")
}

#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_repo() {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect("postgresql://pog_user:pog_pass@localhost:5432/pog_server")
            .await
            .expect("unable to connect to database");
        let repo = PostgresAdminRepository::new(pool);
        let mut settings = repo.get().await.unwrap();
        println!("{:?}", settings);
        settings.welcome_channel = String::new();
        repo.update(settings).await.unwrap();
    }
}
