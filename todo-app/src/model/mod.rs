use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Formatter;

pub mod todo;

pub struct DateTimeRfc3339(pub String);

impl From<DateTime<Utc>> for DateTimeRfc3339 {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt.to_rfc3339())
    }
}

impl fmt::Display for DateTimeRfc3339 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
