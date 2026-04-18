use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Parse(String),
    InvalidTimeRange(String),
    InvalidFieldFilter(String),
    InvalidTransformOp(String),
    InvalidFormat(String),
    Unknown(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(s) => write!(f, "Parse error: {}", s),
            AppError::InvalidTimeRange(s) => write!(f, "Invalid time range: {}", s),
            AppError::InvalidFieldFilter(s) => write!(f, "Invalid field filter: {}", s),
            AppError::InvalidTransformOp(s) => write!(f, "Invalid transform op: {}", s),
            AppError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            AppError::Unknown(s) => write!(f, "Unknown error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Unknown(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Unknown(s.to_string())
    }
}
