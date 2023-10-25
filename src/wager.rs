use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Wager {
    pub time: String,
    pub offering: String,
    pub accepting: String,
    pub wager: String,
    pub outcome: String,
    pub status: WagerStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WagerStatus {
    Open = 0,
    Paid = 1,
    Cancelled = 2,
    NoBet = 3,
}

impl WagerStatus {
    pub fn as_i16(&self) -> i16 {
        self.clone() as i16
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
    let test_cases: Vec<WagerStatus> = vec![WagerStatus::Open, WagerStatus::Paid, WagerStatus::Cancelled, WagerStatus::NoBet];
    for case in test_cases {
        let i16_value = case.as_i16();
        assert_eq!(case, WagerStatus::from_i16(i16_value));
    }
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
