#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::rate::RateFilter;
    use crate::rate_config::RateConfig;

    fn make_entry(ts_ms: i64) -> LogEntry {
        LogEntry {
            timestamp_ms: ts_ms,
            raw: format!("{{\"ts\": {}}}", ts_ms),
            fields: Default::default(),
        }
    }

    #[test]
    fn test_accept_within_limit() {
        let config = RateConfig::new(3, 1_000);
        let mut filter = RateFilter::new(config);
        assert!(filter.accept(&make_entry(1000)));
        assert!(filter.accept(&make_entry(1200)));
        assert!(filter.accept(&make_entry(1400)));
    }

    #[test]
    fn test_reject_over_limit() {
        let config = RateConfig::new(2, 1_000);
        let mut filter = RateFilter::new(config);
        assert!(filter.accept(&make_entry(1000)));
        assert!(filter.accept(&make_entry(1100)));
        assert!(!filter.accept(&make_entry(1200)));
    }

    #[test]
    fn test_window_slides() {
        let config = RateConfig::new(2, 1_000);
        let mut filter = RateFilter::new(config);
        assert!(filter.accept(&make_entry(1000)));
        assert!(filter.accept(&make_entry(1100)));
        // 2100 evicts entries before 1100, so window has only 1100
        assert!(filter.accept(&make_entry(2100)));
    }

    #[test]
    fn test_apply_filters_entries() {
        let config = RateConfig::new(2, 1_000);
        let mut filter = RateFilter::new(config);
        let entries = vec![
            make_entry(1000),
            make_entry(1100),
            make_entry(1200),
            make_entry(1300),
        ];
        let result = filter.apply(entries);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_current_rate() {
        let config = RateConfig::new(5, 1_000);
        let mut filter = RateFilter::new(config);
        filter.accept(&make_entry(1000));
        filter.accept(&make_entry(1100));
        assert_eq!(filter.current_rate(), 2);
    }

    #[test]
    fn test_per_second_constructor() {
        let config = RateConfig::per_second(10);
        assert_eq!(config.max_count, 10);
        assert_eq!(config.window_ms, 1_000);
    }

    #[test]
    fn test_per_minute_constructor() {
        let config = RateConfig::per_minute(100);
        assert_eq!(config.max_count, 100);
        assert_eq!(config.window_ms, 60_000);
    }
}
