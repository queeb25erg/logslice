use std::fmt;

#[derive(Debug)]
pub enum LogSliceError {
    ParseError(String),
    TimeRangeError(String),
    FieldFilterError(String),
    IoError(std::io::Error),
    FormatError(String),
    InvalidArgument(String),
}

impl fmt::Display for LogSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogSliceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            LogSliceError::TimeRangeError(msg) => write!(f, "Time range error: {}", msg),
            LogSliceError::FieldFilterError(msg) => write!(f, "Field filter error: {}", msg),
            LogSliceError::IoError(e) => write!(f, "IO error: {}", e),
            LogSliceError::FormatError(msg) => write!(f, "Format error: {}", msg),
            LogSliceError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for LogSliceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogSliceError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LogSliceError {
    fn from(e: std::io::Error) -> Self {
        LogSliceError::IoError(e)
    }
}

impl From<serde_json::Error> for LogSliceError {
    fn from(e: serde_json::Error) -> Self {
        LogSliceError::ParseError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LogSliceError>;
