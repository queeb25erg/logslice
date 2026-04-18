#[cfg(test)]
mod tests {
    use super::super::error::{LogSliceError, Result};
    use std::io;

    #[test]
    fn test_parse_error_display() {
        let e = LogSliceError::ParseError("unexpected token".to_string());
        assert_eq!(format!("{}", e), "Parse error: unexpected token");
    }

    #[test]
    fn test_time_range_error_display() {
        let e = LogSliceError::TimeRangeError("invalid range".to_string());
        assert_eq!(format!("{}", e), "Time range error: invalid range");
    }

    #[test]
    fn test_field_filter_error_display() {
        let e = LogSliceError::FieldFilterError("missing field".to_string());
        assert_eq!(format!("{}", e), "Field filter error: missing field");
    }

    #[test]
    fn test_io_error_display() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let e = LogSliceError::IoError(io_err);
        assert!(format!("{}", e).contains("IO error"));
    }

    #[test]
    fn test_format_error_display() {
        let e = LogSliceError::FormatError("unsupported format".to_string());
        assert_eq!(format!("{}", e), "Format error: unsupported format");
    }

    #[test]
    fn test_invalid_argument_display() {
        let e = LogSliceError::InvalidArgument("--from required".to_string());
        assert_eq!(format!("{}", e), "Invalid argument: --from required");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let e: LogSliceError = io_err.into();
        assert!(matches!(e, LogSliceError::IoError(_)));
    }

    #[test]
    fn test_result_ok() {
        let r: Result<i32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let r: Result<i32> = Err(LogSliceError::ParseError("bad".to_string()));
        assert!(r.is_err());
    }

    #[test]
    fn test_source_io_error() {
        use std::error::Error;
        let io_err = io::Error::new(io::ErrorKind::Other, "oops");
        let e = LogSliceError::IoError(io_err);
        assert!(e.source().is_some());
    }

    #[test]
    fn test_source_non_io_error() {
        use std::error::Error;
        let e = LogSliceError::ParseError("x".to_string());
        assert!(e.source().is_none());
    }
}
