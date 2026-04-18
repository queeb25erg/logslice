use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum TimeRangeError {
    InvalidFormat(String),
    StartAfterEnd,
}

impl fmt::Display for TimeRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeRangeError::InvalidFormat(s) => write!(f, "Invalid time format: {}", s),
            TimeRangeError::StartAfterEnd => write!(f, "Start time must be before end time"),
        }
    }
}

impl TimeRange {
    pub fn new(
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Self, TimeRangeError> {
        if let (Some(s), Some(e)) = (start, end) {
            if s > e {
                return Err(TimeRangeError::StartAfterEnd);
            }
            return Ok(TimeRange { start: Some(s), end: Some(e) });
        }
        Ok(TimeRange { start, end })
    }

    pub fn contains(&self, ts: &DateTime<Utc>) -> bool {
        let after_start = self.start.map_or(true, |s| *ts >= s);
        let before_end = self.end.map_or(true, |e| *ts <= e);
        after_start && before_end
    }

    pub fn is_unbounded(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }
}

pub fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, TimeRangeError> {
    // Try RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    // Try common log format: "2024-01-15 10:30:00"
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(dt, Utc));
    }
    // Try date only: "2024-01-15"
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let dt = d.and_hms_opt(0, 0, 0).unwrap();
        return Ok(DateTime::from_naive_utc_and_offset(dt, Utc));
    }
    Err(TimeRangeError::InvalidFormat(s.to_string()))
}
