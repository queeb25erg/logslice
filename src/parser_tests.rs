#[cfg(test)]
mod tests {
    use crate::parser::{parse_line, parse_lines, ParseError};

    #[test]
    fn test_parse_valid_line() {
        let line = r#"{"timestamp":"2024-01-15T10:00:00Z","level":"INFO","message":"started"}"#;
        let entry = parse_line(line).unwrap();
        assert_eq!(entry.timestamp, "2024-01-15T10:00:00Z");
        assert_eq!(entry.fields.get("level").unwrap(), "INFO");
        assert_eq!(entry.fields.get("message").unwrap(), "started");
    }

    #[test]
    fn test_parse_ts_alias() {
        let line = r#"{"ts":"2024-01-15T11:00:00Z","level":"DEBUG"}"#;
        let entry = parse_line(line).unwrap();
        assert_eq!(entry.timestamp, "2024-01-15T11:00:00Z");
    }

    #[test]
    fn test_parse_time_alias() {
        let line = r#"{"time":"2024-01-15T12:00:00Z","level":"WARN"}"#;
        let entry = parse_line(line).unwrap();
        assert_eq!(entry.timestamp, "2024-01-15T12:00:00Z");
    }

    #[test]
    fn test_parse_empty_line() {
        assert_eq!(parse_line(""), Err(ParseError::EmptyLine));
        assert_eq!(parse_line("   "), Err(ParseError::EmptyLine));
    }

    #[test]
    fn test_parse_invalid_json() {
        let result = parse_line("not json");
        assert!(matches!(result, Err(ParseError::InvalidJson(_))));
    }

    #[test]
    fn test_parse_missing_timestamp() {
        let line = r#"{"level":"INFO","message":"no time"}"#;
        assert_eq!(parse_line(line), Err(ParseError::MissingTimestamp));
    }

    #[test]
    fn test_parse_numeric_field() {
        let line = r#"{"timestamp":"2024-01-15T10:00:00Z","duration":42}"#;
        let entry = parse_line(line).unwrap();
        assert_eq!(entry.fields.get("duration").unwrap(), "42");
    }

    #[test]
    fn test_parse_lines_skips_empty() {
        let input = vec![
            r#"{"timestamp":"2024-01-15T10:00:00Z","level":"INFO"}"#,
            "",
            r#"{"timestamp":"2024-01-15T11:00:00Z","level":"ERROR"}"#,
        ];
        let entries = parse_lines(input.into_iter(), true);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_parse_lines_skip_errors_true() {
        let input = vec![
            r#"{"timestamp":"2024-01-15T10:00:00Z","level":"INFO"}"#,
            "bad line",
        ];
        let entries = parse_lines(input.into_iter(), true);
        assert_eq!(entries.len(), 1);
    }
}
