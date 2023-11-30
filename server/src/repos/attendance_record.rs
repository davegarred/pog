use crate::discord_id::DiscordId;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone, PartialEq)]
pub struct AttendanceRecords(pub Vec<AttendanceRecord>);

#[derive(Debug, Clone, PartialEq)]
pub struct AttendanceRecord {
    pub owner_id: DiscordId,
    pub weeks: u8,
    pub games: u8,
}

impl AttendanceRecords {
    pub fn position_and_values(&self, user_id: &DiscordId) -> Option<(u8, AttendanceRecord)> {
        for (i, record) in self.0.iter().enumerate() {
            if &record.owner_id == user_id {
                return Some((i as u8, record.clone()));
            }
        }
        None
    }
}

impl From<(i64, i64, i64)> for AttendanceRecord {
    fn from((id, weeks, games): (i64, i64, i64)) -> Self {
        let owner_id: DiscordId = id.into();
        Self {
            owner_id,
            weeks: weeks as u8,
            games: games as u8,
        }
    }
}

impl From<PgRow> for AttendanceRecord {
    fn from(row: PgRow) -> Self {
        (&row).into()
    }
}

impl From<&PgRow> for AttendanceRecord {
    fn from(row: &PgRow) -> Self {
        let owner: i64 = row.get("owner");
        let owner_id = owner.into();
        let weeks: i64 = row.get("weeks");
        let games: i64 = row.get("games");
        Self {
            owner_id,
            weeks: weeks as u8,
            games: games as u8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WeeklyAttendanceRecord {
    pub attendance: Vec<(String, Vec<DiscordId>)>,
}

impl From<Vec<(String, Vec<DiscordId>)>> for WeeklyAttendanceRecord {
    fn from(attendance: Vec<(String, Vec<DiscordId>)>) -> Self {
        Self { attendance }
    }
}

impl From<Vec<PgRow>> for WeeklyAttendanceRecord {
    fn from(rows: Vec<PgRow>) -> Self {
        let mut attendance = vec![];
        let mut last_date = String::new();
        let mut current_attendees: Vec<DiscordId> = Vec::new();
        for row in rows {
            let date: String = row.get("date");
            let owner: i64 = row.get("owner");
            let owner_id = owner.into();

            if date != last_date {
                if !current_attendees.is_empty() {
                    attendance.push((last_date.to_string(), current_attendees.clone()));
                }
                last_date = date;
                current_attendees.clear();
            }
            current_attendees.push(owner_id);
        }
        attendance.push((last_date, current_attendees));
        Self { attendance }
    }
}
