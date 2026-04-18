#[cfg(test)]
mod tests {
    use super::super::stats::LogStats;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(ts: Option<&str>, level: Option<&str>) -> LogEntry {
        let mut fields = HashMap::new();
        if let Some(l) = level {
            fields.insert("level".to_string(), l.to_string());
        }
        LogEntry {
            timestamp: ts.map(|s| s.to_string()),
            fields,
            raw: String::new(),
        }
    }

    #[test]
    fn test_empty_stats() {
        let s = LogStats::new();
        assert_eq!(s.total, 0);
        assert!(s.earliest.is_none());
        assert!(s.latest.is_none());
    }

    #[test]
    fn test_record_increments_total() {
        let mut s = LogStats::new();
        s.record(&make_entry(Some("2024-01-01T00:00:00Z"), Some("info")));
        s.record(&make_entry(Some("2024-01-02T00:00:00Z"), Some("error")));
        assert_eq!(s.total, 2);
    }

    #[test]
    fn test_by_level_counts() {
        let mut s = LogStats::new();
        s.record(&make_entry(None, Some("info")));
        s.record(&make_entry(None, Some("info")));
        s.record(&make_entry(None, Some("error")));
        assert_eq!(s.by_level["info"], 2);
        assert_eq!(s.by_level["error"], 1);
    }

    #[test]
    fn test_earliest_latest() {
        let mut s = LogStats::new();
        s.record(&make_entry(Some("2024-03-01T00:00:00Z"), None));
        s.record(&make_entry(Some("2024-01-01T00:00:00Z"), None));
        s.record(&make_entry(Some("2024-06-01T00:00:00Z"), None));
        assert_eq!(s.earliest.as_deref(), Some("2024-01-01T00:00:00Z"));
        assert_eq!(s.latest.as_deref(), Some("2024-06-01T00:00:00Z"));
    }

    #[test]
    fn test_summary_contains_total() {
        let mut s = LogStats::new();
        s.record(&make_entry(Some("2024-01-01T00:00:00Z"), Some("warn")));
        let summary = s.summary();
        assert!(summary.contains("Total entries"));
        assert!(summary.contains('1'));
        assert!(summary.contains("warn"));
    }
}
