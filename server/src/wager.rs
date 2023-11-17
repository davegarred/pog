use chrono::NaiveDate;
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
    pub expected_settle_date: Option<NaiveDate>,
}

impl Display for Wager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let close = match (self.expected_settle_date, self.status) {
            (Some(date), WagerStatus::Open) => format!(" (settles: {})", date.format("%b %e")),
            _ => "".to_string(),
        };
        write!(
            f,
            "{} vs {}, wager: {} - {}{}",
            self.offering, self.accepting, self.wager, self.outcome, close
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
            "{} vs {}, wager: {} - {}",
            offering, accepting, self.wager, self.outcome
        )
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WagerStatus {
    Open = 0,
    Paid = 1,
    OfferingWon = 2,
    AcceptingWon = 3,
    NoBet = 4,
}

impl WagerStatus {
    pub fn as_i16(&self) -> i16 {
        *self as i16
    }
    pub fn from_i16(val: i16) -> Self {
        match val {
            0 => WagerStatus::Open,
            1 => WagerStatus::Paid,
            2 => WagerStatus::OfferingWon,
            3 => WagerStatus::AcceptingWon,
            4 => WagerStatus::NoBet,
            v => panic!("attempt to convert {} to WagerStatus", v),
        }
    }
}

#[test]
fn test_wager_status() {
    let test_cases: Vec<WagerStatus> = vec![
        WagerStatus::Open,
        WagerStatus::Paid,
        WagerStatus::OfferingWon,
        WagerStatus::AcceptingWon,
        WagerStatus::NoBet,
    ];
    for case in test_cases {
        let i16_value = case.as_i16();
        assert_eq!(case, WagerStatus::from_i16(i16_value));
    }
}

#[test]
fn test_format() {
    let wager = Wager {
        wager_id: 109,
        time: "".to_string(),
        offering: "Harx".to_string(),
        resolved_offering_user: Some(1234567890.into()),
        accepting: "Woody".to_string(),
        resolved_accepting_user: None,
        wager: "$20".to_string(),
        outcome: "Cowboys over the Raiders".to_string(),
        status: WagerStatus::Open,
        expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
    };
    assert_eq!(
        wager.to_string(),
        "Harx vs Woody, wager: $20 - Cowboys over the Raiders (settles: May  5)"
    );
}

#[test]
fn test_format_no_expected_settlement_date() {
    let wager = Wager {
        wager_id: 109,
        time: "".to_string(),
        offering: "Harx".to_string(),
        resolved_offering_user: Some(1234567890.into()),
        accepting: "Woody".to_string(),
        resolved_accepting_user: None,
        wager: "$20".to_string(),
        outcome: "Cowboys over the Raiders".to_string(),
        status: WagerStatus::Open,
        expected_settle_date: None,
    };
    assert_eq!(
        wager.to_string(),
        "Harx vs Woody, wager: $20 - Cowboys over the Raiders"
    );
}

#[test]
fn test_format_paid() {
    let wager = Wager {
        wager_id: 109,
        time: "".to_string(),
        offering: "Harx".to_string(),
        resolved_offering_user: Some(1234567890.into()),
        accepting: "Woody".to_string(),
        resolved_accepting_user: None,
        wager: "$20".to_string(),
        outcome: "Cowboys over the Raiders".to_string(),
        status: WagerStatus::Paid,
        expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
    };
    assert_eq!(
        wager.to_string(),
        "Harx vs Woody, wager: $20 - Cowboys over the Raiders"
    );
}
