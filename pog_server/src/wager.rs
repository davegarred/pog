use std::fmt::{Display, Formatter};

use crate::discord_id::DiscordId;

#[derive(Debug, Clone, PartialEq)]
pub struct Wager {
    pub wager_id: u32,
    pub time: String,
    pub offering: String,
    pub resolved_offering_user: Option<DiscordId>,
    pub accepting: String,
    pub resolved_accepting_user: Option<DiscordId>,
    pub wager: String,
    pub outcome: String,
    pub status: WagerStatus,
}

impl Display for Wager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs {}, {} - {}",
            self.offering, self.accepting, self.wager, self.outcome
        )
    }
}

impl Wager {
    pub fn to_resolved_string(&self) -> String {
        let offering = match &self.resolved_offering_user {
            Some(id) => id.to_string(),
            None => self.offering.to_string(),
        };
        let accepting = match &self.resolved_accepting_user {
            Some(id) => id.to_string(),
            None => self.accepting.to_string(),
        };
        format!(
            "{} vs {}, {} - {}",
            offering, accepting, self.wager, self.outcome
        )
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WagerStatus {
    Open = 0,
    Paid = 1,
    Cancelled = 2,
    NoBet = 3,
}

impl WagerStatus {
    pub fn as_i16(&self) -> i16 {
        *self as i16
    }
    pub fn from_i16(val: i16) -> Self {
        match val {
            0 => WagerStatus::Open,
            1 => WagerStatus::Paid,
            2 => WagerStatus::Cancelled,
            3 => WagerStatus::NoBet,
            v => panic!("attempt to convert {} to WagerStatus", v),
        }
    }
}

#[test]
fn test_wager_status() {
    let test_cases: Vec<WagerStatus> = vec![
        WagerStatus::Open,
        WagerStatus::Paid,
        WagerStatus::Cancelled,
        WagerStatus::NoBet,
    ];
    for case in test_cases {
        let i16_value = case.as_i16();
        assert_eq!(case, WagerStatus::from_i16(i16_value));
    }
}
