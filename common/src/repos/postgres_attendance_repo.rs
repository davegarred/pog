use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::repos::attendance_record::{AttendanceRecords, WeeklyAttendanceRecord};
use crate::repos::AttendanceRepository;
use sqlx::{Pool, Postgres};

const COMBINED_ATTENDANCE_QUERY: &str = r#"
SELECT ow.owner    owner,
       count(week) weeks,
       sum(games::int)  games
FROM (SELECT teams.owner owner,
             fa.week     week,
             count(week) games
      FROM ff_teams teams
               FULL OUTER JOIN ff_attendance fa on teams.owner = fa.owner
      WHERE season = '2024'
      GROUP BY teams.owner, fa.week) ow
GROUP BY ow.owner
ORDER BY weeks DESC, games DESC;
"#;

const WEEKLY_ATTENDANCE_QUERY: &str = r#"
SELECT attendance.date  date,
       teams.owner      owner,
       teams.owner_name owner_name
FROM ff_teams teams,
     ff_attendance attendance
WHERE teams.owner = attendance.owner
  AND attendance.week = $1
  AND season = '2024'
GROUP BY attendance.date, teams.owner, teams.owner_name
ORDER BY date;
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

impl AttendanceRepository for PostgresAttendanceRepository {
    async fn combined_attendance(&self) -> Result<AttendanceRecords, Error> {
        let mut result = Vec::default();
        for row in sqlx::query(COMBINED_ATTENDANCE_QUERY)
            .fetch_all(&self.pool)
            .await?
        {
            result.push(row.into());
        }
        Ok(AttendanceRecords(result))
    }

    async fn week_attendance(
        &self,
        week: u8,
        interested_owner: &Option<DiscordId>,
    ) -> Result<WeeklyAttendanceRecord, Error> {
        let results = sqlx::query(WEEKLY_ATTENDANCE_QUERY)
            .bind(week as i32)
            .fetch_all(&self.pool)
            .await?;
        Ok(WeeklyAttendanceRecord::from_query_result(
            interested_owner,
            results,
        ))
    }
}

#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod test {
    use crate::repos::{new_db_pool, AttendanceRepository, PostgresAttendanceRepository};

    #[tokio::test]
    async fn combined_attendance() {
        let db_pool = new_db_pool("postgresql://pog_user:pog_pass@127.0.0.1:5432/pog_server").await;
        let repo = PostgresAttendanceRepository::new(db_pool);
        repo.combined_attendance().await.unwrap();
    }
    #[tokio::test]
    async fn week_attendance() {
        let db_pool = new_db_pool("postgresql://pog_user:pog_pass@127.0.0.1:5432/pog_server").await;
        let repo = PostgresAttendanceRepository::new(db_pool);

        let result = repo.week_attendance(1, &None).await.unwrap();
        assert_eq!(4, result.attendance.len());
    }
}
