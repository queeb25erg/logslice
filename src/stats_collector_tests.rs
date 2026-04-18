#[cfg(test)]
mod tests {
    use super::super::stats_collector::StatsCollector;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn entry(ts: &str, level: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("level".to_string(), level.to_string());
        LogEntry {
            timestamp: Some(ts.to_string()),
            fields,
            raw: format!("{{\"ts\":\"{}\",\"level\":\"{}\"}}", ts, level),
        }
    }

    #[test]
    fn test_disabled_collector_records_nothing() {
        let mut c = StatsCollector::new(false);
        c.observe(&entry("2024-01-01T00:00:00Z", "info"));
        assert_eq!(c.stats().total, 0);
    }

    #[test]
    fn test_enabled_collector_records() {
        let mut c = StatsCollector::new(true);
        c.observe(&entry("2024-01-01T00:00:00Z", "info"));
        c.observe(&entry("2024-01-02T00:00:00Z", "error"));
        assert_eq!(c.stats().total, 2);
    }

    #[test]
    fn test_observe_all() {
        let mut c = StatsCollector::new(true);
        let entries = vec![
            entry("2024-01-01T00:00:00Z", "debug"),
            entry("2024-01-02T00:00:00Z", "info"),
            entry("2024-01-03T00:00:00Z", "info"),
        ];
        c.observe_all(&entries);
        assert_eq!(c.stats().total, 3);
        assert_eq!(c.stats().by_level["info"], 2);
        assert_eq!(c.stats().by_level["debug"], 1);
    }

    #[test]
    fn test_earliest_latest_via_collector() {
        let mut c = StatsCollector::new(true);
        c.observe(&entry("2024-06-01T00:00:00Z", "info"));
        c.observe(&entry("2024-01-01T00:00:00Z", "info"));
        assert_eq!(
            c.stats().earliest.as_deref(),
            Some("2024-01-01T00:00:00Z")
        );
        assert_eq!(
            c.stats().latest.as_deref(),
            Some("2024-06-01T00:00:00Z")
        );
    }
}
