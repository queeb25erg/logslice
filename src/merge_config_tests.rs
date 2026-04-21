#[cfg(test)]
mod tests {
    use crate::merge_config::MergeConfig;

    #[test]
    fn test_default_config() {
        let config = MergeConfig::default();
        assert_eq!(config.timestamp_field, "timestamp");
        assert!(!config.dedup);
    }

    #[test]
    fn test_custom_timestamp_field() {
        let config = MergeConfig::new("time");
        assert_eq!(config.timestamp_field, "time");
        assert!(!config.dedup);
    }

    #[test]
    fn test_with_dedup_enabled() {
        let config = MergeConfig::new("ts").with_dedup(true);
        assert_eq!(config.timestamp_field, "ts");
        assert!(config.dedup);
    }

    #[test]
    fn test_with_dedup_disabled() {
        let config = MergeConfig::new("ts").with_dedup(false);
        assert!(!config.dedup);
    }

    #[test]
    fn test_clone() {
        let config = MergeConfig::new("created_at").with_dedup(true);
        let cloned = config.clone();
        assert_eq!(cloned.timestamp_field, "created_at");
        assert!(cloned.dedup);
    }
}
