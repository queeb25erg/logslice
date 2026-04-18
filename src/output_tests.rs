#[cfg(test)]
mod tests {
    use crate::output::{OutputFormat, OutputWriter};
    use crate::log_entry::LogEntry;

    fn make_entry(raw: &str) -> LogEntry {
        LogEntry::parse(raw).unwrap_or_else(|| LogEntry {
            raw: raw.to_string(),
            timestamp: None,
            fields: std::collections::HashMap::new(),
        })
    }

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("JSON"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("raw"), Some(OutputFormat::Raw));
        assert_eq!(OutputFormat::from_str("pretty"), Some(OutputFormat::Pretty));
        assert_eq!(OutputFormat::from_str("csv"), None);
    }

    #[test]
    fn test_write_entry_raw_increments_count() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = OutputWriter::new(buf, OutputFormat::Raw);
        let entry = make_entry(r#"{"level":"info","msg":"hello"}"}"#);
        writer.write_entry(&entry).unwrap();
        assert_eq!(writer.count(), 1);
    }

    #[test]
    fn test_write_all_counts_entries() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = OutputWriter::new(buf, OutputFormat::Json);
        let entries = vec![
            make_entry(r#"{"level":"info","msg":"a"}"}"#),
            make_entry(r#"{"level":"warn","msg":"b"}"}"#),
            make_entry(r#"{"level":"error","msg":"c"}"}"#),
        ];
        writer.write_all(&entries).unwrap();
        assert_eq!(writer.count(), 3);
    }

    #[test]
    fn test_write_entry_raw_output_contains_content() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = OutputWriter::new(buf, OutputFormat::Raw);
        let raw = r#"{"level":"info","msg":"test"}"}"#;
        let entry = make_entry(raw);
        writer.write_entry(&entry).unwrap();
        let output = String::from_utf8(writer.writer).unwrap();
        assert!(output.contains("info"));
    }

    #[test]
    fn test_pretty_format_includes_timestamp_label() {
        let buf: Vec<u8> = Vec::new();
        let mut writer = OutputWriter::new(buf, OutputFormat::Pretty);
        let entry = make_entry("plain log line with no timestamp");
        writer.write_entry(&entry).unwrap();
        let output = String::from_utf8(writer.writer).unwrap();
        assert!(output.contains("<no timestamp>"));
    }
}
