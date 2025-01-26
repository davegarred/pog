use crate::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct DiscordId(u64);

impl DiscordId {
    pub fn value(&self) -> i64 {
        self.0 as i64
    }

    pub fn str_value(&self) -> String {
        format!("{}", self.0)
    }

    pub fn attempt_from_str(value: &str) -> Option<Self> {
        let value = str::trim(value);
        if value.starts_with("<@") && value.ends_with('>') {
            let len = value.len();
            let attempt = &value[2..len - 1];
            return Self::from_raw_str(attempt);
        }
        None
    }

    pub fn require_from_str(value: &str) -> Result<Self, Error> {
        let value = str::trim(value);
        if value.starts_with("<@") && value.ends_with('>') {
            let len = value.len();
            let attempt = &value[2..len - 1];
            if let Some(user_id) = Self::from_raw_str(attempt) {
                return Ok(user_id);
            }
        }
        Err(format!("unable to parse discord id: {}", value)
            .as_str()
            .into())
    }

    pub fn from_raw_str(value: &str) -> Option<Self> {
        match value.parse::<u64>() {
            Ok(value) => Some(DiscordId(value)),
            Err(_) => None,
        }
    }
}

impl From<i64> for DiscordId {
    fn from(value: i64) -> Self {
        DiscordId(value as u64)
    }
}

impl std::fmt::Display for DiscordId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.0)
    }
}

#[test]
fn test_attempt_from_str() {
    assert_eq!(Some(DiscordId(11)), DiscordId::attempt_from_str("<@11>"));
    assert_eq!(
        Some(DiscordId(11)),
        DiscordId::attempt_from_str(" <@11> \n")
    );
}

pub fn combine_user_payload(user: &str, id: Option<DiscordId>) -> String {
    match id {
        Some(id) => format!("{}|{}", id.0, user),
        None => user.to_string(),
    }
}
pub fn split_combined_user_payload(value: &str) -> (String, Option<DiscordId>) {
    if let Some(delim_pos) = value.find('|') {
        let id_sec = &value[..delim_pos];
        match id_sec.parse::<u64>() {
            Ok(id) => return (value[delim_pos + 1..].to_string(), Some(DiscordId(id))),
            Err(_) => return (value.to_string(), None),
        }
    };
    (value.to_string(), None)
}
#[test]
fn test_combined_user_payload() {
    assert_eq!("11|Harx", combine_user_payload("Harx", Some(DiscordId(11))));
    assert_eq!("Harx", combine_user_payload("Harx", None));
    assert_eq!(
        ("Harx".to_string(), Some(DiscordId(11))),
        split_combined_user_payload("11|Harx")
    );
    assert_eq!(
        ("Harx".to_string(), None),
        split_combined_user_payload("Harx")
    );
    assert_eq!(
        ("Harx|orsumthin".to_string(), None),
        split_combined_user_payload("Harx|orsumthin")
    );
}
