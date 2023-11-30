mod attendance_record;
mod attendance_repository;
mod postgres_attendance_repo;
mod postgres_wager_repository;
mod wager_repository;

pub use attendance_record::{AttendanceRecord, AttendanceRecords, WeeklyAttendanceRecord};
pub use attendance_repository::{AttendanceRepository, InMemoryAttendanceRepository};
pub use postgres_attendance_repo::PostgresAttendanceRepository;
pub use postgres_wager_repository::PostgresWagerRepo;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
pub use wager_repository::{InMemWagerRepository, WagerRepository};

pub async fn new_db_pool(connection_string: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(2)
        .connect(connection_string)
        .await
        .expect("unable to connect to database")
}
