use sqlx::{Pool, Postgres};

use crate::error::Error;
use crate::repos::attendance_record::AttendanceRecords;
use crate::repos::AttendanceRepository;

const ATTENDANCE_QUERY: &str = r#"
SELECT ow.owner    owner,
       count(week) weeks,
       sum(games::int)  games
FROM (SELECT teams.owner owner,
             fa.week     week,
             count(week)    games
      FROM ff_teams teams
               FULL OUTER JOIN ff_attendance fa on teams.owner = fa.owner
      GROUP BY teams.owner, fa.week) ow
GROUP BY ow.owner
ORDER BY weeks DESC, games DESC;
"#;

#[derive(Debug, Clone)]
pub struct PostgresAttendanceRepository {
    pool: Pool<Postgres>,
}

impl PostgresAttendanceRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AttendanceRepository for PostgresAttendanceRepository {
    async fn attendance(&self) -> Result<AttendanceRecords, Error> {
        self.query(ATTENDANCE_QUERY).await
    }
}

impl PostgresAttendanceRepository {
    async fn query(&self, query: &str) -> Result<AttendanceRecords, Error> {
        let mut result = Vec::default();
        for row in sqlx::query(query).fetch_all(&self.pool).await? {
            result.push(row.into());
        }
        Ok(AttendanceRecords(result))
    }
}

#[cfg(test)]
mod test {
    use crate::repos::new_db_pool;
    use crate::repos::{AttendanceRepository, PostgresAttendanceRepository};

    #[tokio::test]
    async fn repo() {
        let db_pool = new_db_pool("postgresql://pog_user:pog_pass@127.0.0.1:5432/pog_server").await;
        let repo = PostgresAttendanceRepository::new(db_pool);
        repo.attendance().await.unwrap();
    }
}
