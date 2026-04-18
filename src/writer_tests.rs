#[cfg(test)]
mod tests {
    use super::super::writer::Writer;
    use crate::format::OutputFormat;
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
    fn test_writer_plain_single_entry() {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf, OutputFormat::Plain);
        let entry = make_entry("2024-01-01T00:00:00Z", "INFO", "hello");
        writer.write_entry(&entry).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("INFO"));
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_writer_csv_includes_header() {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf, OutputFormat::Csv);
        let entry = make_entry("2024-01-01T00:00:00Z", "ERROR", "fail");
        writer.write_entry(&entry).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.starts_with("timestamp,level,message"));
    }

    #[test]
    fn test_writer_csv_header_once() {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf, OutputFormat::Csv);
        let e1 = make_entry("2024-01-01T00:00:00Z", "INFO", "a");
        let e2 = make_entry("2024-01-01T00:00:01Z", "WARN", "b");
        writer.write_entry(&e1).unwrap();
        writer.write_entry(&e2).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output.matches("timestamp,level,message").count(), 1);
    }

    #[test]
    fn test_writer_returns_count() {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf, OutputFormat::Plain);
        let entries = vec![
            make_entry("2024-01-01T00:00:00Z", "INFO", "a"),
            make_entry("2024-01-01T00:00:01Z", "INFO", "b"),
            make_entry("2024-01-01T00:00:02Z", "INFO", "c"),
        ];
        let count = writer.write_entries(&entries).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_writer_json_format() {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf, OutputFormat::Json);
        let entry = make_entry("2024-01-01T00:00:00Z", "DEBUG", "test");
        writer.write_entry(&entry).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains('{'));
        assert!(output.contains('}'));
    }
}
