#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::TimeZone;
    use crate::bucket::Bucket;
    use crate::bucket_config::BucketConfig;
    use crate::log_entry::LogEntry;

    fn make_entry(ts_secs: i64, fields: Vec<(&str, &str)>) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            timestamp: chrono::Utc.timestamp_opt(ts_secs, 0).unwrap(),
            fields: map,
            raw: String::new(),
        }
    }

    #[test]
    fn test_bucket_by_field() {
        let config = BucketConfig::by_field("level");
        let mut bucket = Bucket::new(config);
        bucket.insert(make_entry(1000, vec![("level", "info")]));
        bucket.insert(make_entry(1001, vec![("level", "error")]));
        bucket.insert(make_entry(1002, vec![("level", "info")]));
        assert_eq!(bucket.bucket_count(), 2);
        assert_eq!(bucket.total_entries(), 3);
    }

    #[test]
    fn test_bucket_by_field_missing_uses_default() {
        let config = BucketConfig::by_field("level").with_default_key("none");
        let mut bucket = Bucket::new(config);
        bucket.insert(make_entry(1000, vec![]));
        let sorted = bucket.into_sorted();
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].0, "none");
    }

    #[test]
    fn test_bucket_by_interval() {
        let config = BucketConfig::by_interval(60);
        let mut bucket = Bucket::new(config);
        // Both within the same 60s window (0..59)
        bucket.insert(make_entry(10, vec![]));
        bucket.insert(make_entry(45, vec![]));
        // Different window (60..119)
        bucket.insert(make_entry(90, vec![]));
        assert_eq!(bucket.bucket_count(), 2);
        assert_eq!(bucket.total_entries(), 3);
    }

    #[test]
    fn test_into_sorted_returns_ordered_keys() {
        let config = BucketConfig::by_field("service");
        let mut bucket = Bucket::new(config);
        bucket.insert(make_entry(1000, vec![("service", "zebra")]));
        bucket.insert(make_entry(1001, vec![("service", "alpha")]));
        bucket.insert(make_entry(1002, vec![("service", "mango")]));
        let sorted = bucket.into_sorted();
        let keys: Vec<&str> = sorted.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["alpha", "mango", "zebra"]);
    }

    #[test]
    fn test_default_config_uses_60s_interval() {
        let config = BucketConfig::default();
        assert_eq!(config.interval_secs, Some(60));
        assert!(config.field.is_none());
    }

    #[test]
    fn test_empty_bucket() {
        let config = BucketConfig::by_field("level");
        let bucket = Bucket::new(config);
        assert_eq!(bucket.bucket_count(), 0);
        assert_eq!(bucket.total_entries(), 0);
    }
}
