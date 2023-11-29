use crate::metric;
use chrono::{Datelike, Duration, NaiveDate, Utc};

pub fn parse_date(value: &str) -> Option<NaiveDate> {
    if value.is_empty() {
        return None;
    }
    if let Ok(date) = NaiveDate::parse_from_str(value, "%m/%d/%Y") {
        return Some(date);
    }
    let value_with_year = format!("{}/{}", value, Utc::now().year());
    if let Ok(date) = NaiveDate::parse_from_str(value_with_year.as_str(), "%m/%d/%Y") {
        return Some(date);
    }
    try_guessing_year(value, Utc::now().date_naive())
}
fn try_guessing_year(value: &str, today: NaiveDate) -> Option<NaiveDate> {
    let value_with_year = format!("{}/{}", value, Utc::now().year());
    match NaiveDate::parse_from_str(value_with_year.as_str(), "%m/%d/%Y") {
        Ok(date) => {
            let delta = date - today;
            if delta < Duration::zero() {
                Some(date)
            } else {
                Some(date + chrono::Months::new(12))
            }
        }
        Err(_) => {
            metric(|mut m| m.count("parse_date_failure"));
            None
        }
    }
}

#[test]
fn test_date() {
    assert_eq!(
        NaiveDate::from_ymd_opt(2023, 11, 24),
        parse_date("11/24/2023")
    );
    assert_eq!(NaiveDate::from_ymd_opt(2023, 11, 24), parse_date("11/24"));
}

#[test]
fn test_guessing_date() {
    let today: NaiveDate = "2023-11-24".parse().unwrap();
    assert_eq!(
        NaiveDate::from_ymd_opt(2023, 11, 23),
        try_guessing_year("11/23", today)
    );
}

#[test]
fn test_guessing_date_wrong_year() {
    let today: NaiveDate = "2023-11-24".parse().unwrap();
    assert_eq!(
        NaiveDate::from_ymd_opt(2024, 11, 24),
        try_guessing_year("11/24", today)
    );
    assert_eq!(
        NaiveDate::from_ymd_opt(2024, 11, 25),
        try_guessing_year("11/25", today)
    );
}
