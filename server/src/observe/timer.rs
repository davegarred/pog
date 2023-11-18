use chrono::{DateTime, Utc};

use crate::metric;

pub struct Timer {
    name: String,
    time: DateTime<Utc>,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        let time = Utc::now();
        Self {
            name: name.to_string(),
            time,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let now = Utc::now();
        let delta = now - self.time;
        let millis = delta.num_microseconds().ok_or(0).unwrap() as u64;
        metric(|mut m| m.timer(&self.name, millis));
    }
}
