use std::fmt;
use std::str::FromStr;
use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Trace => "TRACE",
            Severity::Debug => "DEBUG",
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::Error => "ERROR",
            Severity::Fatal => "FATAL",
        }
    }

    pub fn matches_min(&self, min: &Severity) -> bool {
        self >= min
    }
}

impl FromStr for Severity {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(Severity::Trace),
            "DEBUG" => Ok(Severity::Debug),
            "INFO" => Ok(Severity::Info),
            "WARN" | "WARNING" => Ok(Severity::Warn),
            "ERROR" | "ERR" => Ok(Severity::Error),
            "FATAL" | "CRITICAL" => Ok(Severity::Fatal),
            other => Err(AppError::Parse(format!("Unknown severity level: {}", other))),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn filter_by_min_severity(level: &str, min: &str) -> Result<bool, AppError> {
    let entry_sev = level.parse::<Severity>()?;
    let min_sev = min.parse::<Severity>()?;
    Ok(entry_sev.matches_min(&min_sev))
}
