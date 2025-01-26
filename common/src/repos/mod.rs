mod admin_repository;
pub mod attendance_record;
pub mod attendance_repository;
mod postgres_admin_repository;
pub mod postgres_attendance_repo;
pub mod postgres_wager_repository;
mod postgres_whois_repository;
pub mod wager_repository;
mod whois_repository;

pub use admin_repository::*;
pub use attendance_repository::*;
pub use postgres_admin_repository::*;
pub use postgres_attendance_repo::*;
pub use postgres_wager_repository::*;
pub use postgres_whois_repository::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
pub use wager_repository::*;
pub use whois_repository::*;

pub async fn new_db_pool(connection_string: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(2)
        .connect(connection_string)
        .await
        .expect("unable to connect to database")
}
