#[cfg(test)]
mod tests {
    use super::super::format::*;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(ts: &str, level: &str, msg: &str) -> LogEntry {
        LogEntry {
            timestamp: ts.to_string(),
            level: level.to_string(),
            message: msg.to_string(),
            fields: HashMap::new(),
            raw: String::new(),
        }
    }

    #[test]
    fn test_format_from_str_json() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
    }

    #[test]
    fn test_format_from_str_plain() {
        assert_eq!(OutputFormat::from_str("plain"), Some(OutputFormat::Plain));
        assert_eq!(OutputFormat::from_str("text"), Some(OutputFormat::Plain));
    }

    #[test]
    fn test_format_from_str_csv() {
        assert_eq!(OutputFormat::from_str("csv"), Some(OutputFormat::Csv));
    }

    #[test]
    fn test_format_from_str_unknown() {
        assert_eq!(OutputFormat::from_str("xml"), None);
    }

    #[test]
    fn test_format_display() {
        assert_eq!(format!("{}", OutputFormat::Json), "json");
        assert_eq!(format!("{}", OutputFormat::Plain), "plain");
        assert_eq!(format!("{}", OutputFormat::Csv), "csv");
    }

    #[test]
    fn test_format_plain_entry() {
        let entry = make_entry("2024-01-01T00:00:00Z", "INFO", "hello world");
        let result = format_entry(&entry, &OutputFormat::Plain);
        assert_eq!(result, "[2024-01-01T00:00:00Z] INFO - hello world");
    }

    #[test]
    fn test_format_csv_entry() {
        let entry = make_entry("2024-01-01T00:00:00Z", "ERROR", "something failed");
        let result = format_entry(&entry, &OutputFormat::Csv);
        assert_eq!(result, "2024-01-01T00:00:00Z,ERROR,something failed");
    }

    #[test]
    fn test_format_csv_escapes_commas() {
        let entry = make_entry("2024-01-01T00:00:00Z", "WARN", "a, b, c");
        let result = format_entry(&entry, &OutputFormat::Csv);
        assert!(result.contains('"'));
    }

    #[test]
    fn test_format_json_entry() {
        let entry = make_entry("2024-01-01T00:00:00Z", "DEBUG", "test msg");
        let result = format_entry(&entry, &OutputFormat::Json);
        assert!(result.contains("\"timestamp\""));
        assert!(result.contains("\"level\""));
        assert!(result.contains("\"message\""));
    }

    #[test]
    fn test_csv_header() {
        assert_eq!(csv_header(), "timestamp,level,message");
    }
}
