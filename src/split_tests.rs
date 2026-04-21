#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::split::Splitter;
    use crate::split_config::SplitConfig;
    use std::collections::HashMap;

    fn make_entry(fields: Vec<(&str, &str)>) -> LogEntry {
        let mut entry = LogEntry::default();
        for (k, v) in fields {
            entry.fields.insert(k.to_string(), v.to_string());
        }
        entry
    }

    #[test]
    fn test_split_by_level() {
        let config = SplitConfig::new("level");
        let splitter = Splitter::new(config);
        let entries = vec![
            make_entry(vec![("level", "info")]),
            make_entry(vec![("level", "error")]),
            make_entry(vec![("level", "info")]),
        ];
        let buckets = splitter.split(entries);
        assert_eq!(buckets["info"].len(), 2);
        assert_eq!(buckets["error"].len(), 1);
    }

    #[test]
    fn test_missing_field_uses_fallback() {
        let config = SplitConfig::new("level");
        let splitter = Splitter::new(config);
        let entries = vec![make_entry(vec![("msg", "no level here")])];
        let buckets = splitter.split(entries);
        assert!(buckets.contains_key("_unknown"));
        assert_eq!(buckets["_unknown"].len(), 1);
    }

    #[test]
    fn test_custom_fallback_key() {
        let config = SplitConfig::new("service").with_fallback_key("misc");
        let splitter = Splitter::new(config);
        let entries = vec![make_entry(vec![])];
        let buckets = splitter.split(entries);
        assert!(buckets.contains_key("misc"));
    }

    #[test]
    fn test_max_buckets_overflow() {
        let config = SplitConfig::new("level").with_max_buckets(1);
        let splitter = Splitter::new(config);
        let entries = vec![
            make_entry(vec![("level", "info")]),
            make_entry(vec![("level", "warn")]),
            make_entry(vec![("level", "error")]),
        ];
        let buckets = splitter.split(entries);
        // Only one real bucket + overflow
        assert!(buckets.contains_key("_overflow"));
        assert_eq!(buckets.len(), 2);
    }

    #[test]
    fn test_bucket_names_sorted() {
        let mut map: std::collections::HashMap<String, Vec<LogEntry>> = HashMap::new();
        map.insert("zebra".to_string(), vec![]);
        map.insert("alpha".to_string(), vec![]);
        map.insert("mango".to_string(), vec![]);
        let names = Splitter::bucket_names(&map);
        assert_eq!(names, vec!["alpha", "mango", "zebra"]);
    }

    #[test]
    fn test_split_config_defaults() {
        let config = SplitConfig::default();
        assert_eq!(config.field, "level");
        assert_eq!(config.fallback_key, "_unknown");
        assert_eq!(config.max_buckets, 0);
        assert_eq!(config.overflow_key, "_overflow");
    }
}
