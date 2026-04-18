#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;

    #[test]
    fn test_parse_basic_json_log() {
        let line = r#"{"timestamp":"2024-01-15T10:00:00Z","level":"INFO","message":"started"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert!(entry.timestamp.is_some());
        assert_eq!(entry.get_field("level").unwrap().as_str().unwrap(), "INFO");
    }

    #[test]
    fn test_parse_alt_timestamp_key() {
        let line = r#"{"ts":"2024-03-01 09:00:00","msg":"hello"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert!(entry.timestamp.is_some());
    }

    #[test]
    fn test_parse_invalid_json() {
        let line = "not json at all";
        assert!(LogEntry::from_json_line(line).is_none());
    }

    #[test]
    fn test_field_matches_string() {
        let line = r#"{"level":"ERROR","service":"auth"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert!(entry.field_matches("level", "ERROR"));
        assert!(!entry.field_matches("level", "INFO"));
    }

    #[test]
    fn test_field_matches_number() {
        let line = r#"{"status":200,"path":"/health"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert!(entry.field_matches("status", "200"));
    }

    #[test]
    fn test_missing_timestamp() {
        let line = r#"{"level":"DEBUG","msg":"no time here"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert!(entry.timestamp.is_none());
    }

    #[test]
    fn test_raw_preserved() {
        let line = r#"{"level":"WARN"}"#;
        let entry = LogEntry::from_json_line(line).unwrap();
        assert_eq!(entry.raw, line);
    }
}
