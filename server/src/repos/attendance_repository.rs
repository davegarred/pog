use crate::error::Error;
use crate::repos::attendance_record::AttendanceRecords;

#[async_trait::async_trait]
pub trait AttendanceRepository {
    async fn attendance(&self) -> Result<AttendanceRecords, Error>;
}

#[derive(Clone, Debug)]
pub struct InMemoryAttendanceRepository {
    pub attendance: AttendanceRecords,
}

impl Default for InMemoryAttendanceRepository {
    fn default() -> Self {
        let attendance = AttendanceRecords(vec![]);
        Self { attendance }
    }
}

#[async_trait::async_trait]
impl AttendanceRepository for InMemoryAttendanceRepository {
    async fn attendance(&self) -> Result<AttendanceRecords, Error> {
        Ok(self.attendance.clone())
    }
}
