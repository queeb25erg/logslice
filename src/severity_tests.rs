#[cfg(test)]
mod tests {
    use super::super::severity::*;
    use std::str::FromStr;

    #[test]
    fn test_ordering() {
        assert!(Severity::Fatal > Severity::Error);
        assert!(Severity::Error > Severity::Warn);
        assert!(Severity::Warn > Severity::Info);
        assert!(Severity::Info > Severity::Debug);
        assert!(Severity::Debug > Severity::Trace);
    }

    #[test]
    fn test_from_str_canonical() {
        assert_eq!(Severity::from_str("INFO").unwrap(), Severity::Info);
        assert_eq!(Severity::from_str("debug").unwrap(), Severity::Debug);
        assert_eq!(Severity::from_str("FATAL").unwrap(), Severity::Fatal);
    }

    #[test]
    fn test_from_str_aliases() {
        assert_eq!(Severity::from_str("WARNING").unwrap(), Severity::Warn);
        assert_eq!(Severity::from_str("ERR").unwrap(), Severity::Error);
        assert_eq!(Severity::from_str("CRITICAL").unwrap(), Severity::Fatal);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(Severity::from_str("VERBOSE").is_err());
        assert!(Severity::from_str("").is_err());
    }

    #[test]
    fn test_matches_min() {
        assert!(Severity::Error.matches_min(&Severity::Warn));
        assert!(Severity::Warn.matches_min(&Severity::Warn));
        assert!(!Severity::Info.matches_min(&Severity::Warn));
    }

    #[test]
    fn test_filter_by_min_severity() {
        assert!(filter_by_min_severity("ERROR", "WARN").unwrap());
        assert!(filter_by_min_severity("WARN", "WARN").unwrap());
        assert!(!filter_by_min_severity("INFO", "WARN").unwrap());
        assert!(!filter_by_min_severity("DEBUG", "ERROR").unwrap());
    }

    #[test]
    fn test_filter_invalid_level() {
        assert!(filter_by_min_severity("NOPE", "WARN").is_err());
        assert!(filter_by_min_severity("INFO", "BADLEVEL").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(Severity::Info.to_string(), "INFO");
        assert_eq!(Severity::Fatal.to_string(), "FATAL");
    }
}
