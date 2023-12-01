use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::repos::attendance_record::{AttendanceRecords, WeeklyAttendanceRecord};

#[async_trait::async_trait]
pub trait AttendanceRepository {
    async fn combined_attendance(&self) -> Result<AttendanceRecords, Error>;
    async fn week_attendance(
        &self,
        week: u8,
        interested_owner: &Option<DiscordId>,
    ) -> Result<WeeklyAttendanceRecord, Error>;
}

#[derive(Clone, Debug)]
pub struct InMemoryAttendanceRepository {
    pub combined_attendance: AttendanceRecords,
    pub weekly_attendance: WeeklyAttendanceRecord,
}

impl Default for InMemoryAttendanceRepository {
    fn default() -> Self {
        let attendance = AttendanceRecords(vec![]);
        let weekly_attendance = Default::default();
        Self {
            combined_attendance: attendance,
            weekly_attendance,
        }
    }
}

#[async_trait::async_trait]
impl AttendanceRepository for InMemoryAttendanceRepository {
    async fn combined_attendance(&self) -> Result<AttendanceRecords, Error> {
        Ok(self.combined_attendance.clone())
    }

    async fn week_attendance(
        &self,
        _week: u8,
        _interested_owner: &Option<DiscordId>,
    ) -> Result<WeeklyAttendanceRecord, Error> {
        Ok(self.weekly_attendance.clone())
    }
}
