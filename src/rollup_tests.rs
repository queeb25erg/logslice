#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::rollup::Rollup;
    use crate::rollup_config::RollupConfig;
    use std::collections::HashMap;

    fn make_entry(level: &str, timestamp: Option<&str>) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("level".to_string(), level.to_string());
        LogEntry {
            raw: format!("level={}", level),
            timestamp: timestamp.map(|s| s.to_string()),
            fields,
        }
    }

    #[test]
    fn test_groups_by_key_field() {
        let entries = vec![
            make_entry("error", None),
            make_entry("info", None),
            make_entry("error", None),
            make_entry("warn", None),
        ];
        let cfg = RollupConfig::new("level");
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&entries);
        let error_bucket = buckets.iter().find(|b| b.key == "error").unwrap();
        assert_eq!(error_bucket.count, 2);
    }

    #[test]
    fn test_sorted_by_count_descending() {
        let entries = vec![
            make_entry("info", None),
            make_entry("error", None),
            make_entry("error", None),
            make_entry("error", None),
            make_entry("warn", None),
            make_entry("warn", None),
        ];
        let cfg = RollupConfig::new("level");
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&entries);
        assert_eq!(buckets[0].key, "error");
        assert_eq!(buckets[0].count, 3);
    }

    #[test]
    fn test_top_n_limits_results() {
        let entries = vec![
            make_entry("info", None),
            make_entry("error", None),
            make_entry("warn", None),
        ];
        let cfg = RollupConfig::new("level").with_top_n(2);
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&entries);
        assert_eq!(buckets.len(), 2);
    }

    #[test]
    fn test_missing_field_uses_default_key() {
        let entry = LogEntry {
            raw: "no level field".to_string(),
            timestamp: None,
            fields: HashMap::new(),
        };
        let cfg = RollupConfig::new("level").with_default_key("unknown");
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&[entry]);
        assert_eq!(buckets[0].key, "unknown");
    }

    #[test]
    fn test_to_summary_entries_includes_count() {
        let entries = vec![make_entry("error", None), make_entry("error", None)];
        let cfg = RollupConfig::new("level");
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&entries);
        let summaries = rollup.to_summary_entries(&buckets);
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].fields.get("count").unwrap(), "2");
    }

    #[test]
    fn test_include_first_timestamp() {
        let entries = vec![make_entry("info", Some("2024-01-01T00:00:00Z"))];
        let cfg = RollupConfig::new("level").with_include_first(true);
        let rollup = Rollup::new(cfg);
        let buckets = rollup.apply(&entries);
        let summaries = rollup.to_summary_entries(&buckets);
        assert!(summaries[0].fields.contains_key("first_timestamp"));
    }
}
