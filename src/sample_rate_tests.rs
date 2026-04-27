#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::sample_rate::SampleRate;
    use crate::sample_rate_config::SampleRateConfig;
    use std::collections::HashMap;

    fn make_entry(level: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("level".to_string(), level.to_string());
        LogEntry {
            timestamp: None,
            message: format!("msg at {}", level),
            fields,
            raw: String::new(),
        }
    }

    #[test]
    fn test_base_rate_emits_every_nth() {
        let cfg = SampleRateConfig::new(3);
        let mut sr = SampleRate::new(cfg);
        let entry = make_entry("info");
        let results: Vec<bool> = (0..9).map(|_| sr.should_emit(&entry)).collect();
        // emits at positions 3, 6, 9 (seen % 3 == 0)
        assert_eq!(results.iter().filter(|&&v| v).count(), 3);
    }

    #[test]
    fn test_always_emit_high_severity() {
        let cfg = SampleRateConfig::new(100)
            .with_always_emit_severity("error");
        let mut sr = SampleRate::new(cfg);
        let error_entry = make_entry("error");
        // Even with rate=100, error entries always emit
        for _ in 0..10 {
            assert!(sr.should_emit(&error_entry));
        }
    }

    #[test]
    fn test_info_not_always_emitted_at_high_rate() {
        let cfg = SampleRateConfig::new(100)
            .with_max_rate(100)
            .with_always_emit_severity("error");
        let mut sr = SampleRate::new(cfg);
        let info_entry = make_entry("info");
        let results: Vec<bool> = (0..99).map(|_| sr.should_emit(&info_entry)).collect();
        // At rate 100, only 0 or 1 should be emitted in first 99
        assert!(results.iter().filter(|&&v| v).count() <= 1);
    }

    #[test]
    fn test_stats_tracking() {
        let cfg = SampleRateConfig::new(2);
        let mut sr = SampleRate::new(cfg);
        let entry = make_entry("debug");
        for _ in 0..10 {
            sr.should_emit(&entry);
        }
        let (seen, emitted) = sr.stats();
        assert_eq!(seen, 10);
        assert_eq!(emitted, 5);
    }

    #[test]
    fn test_effective_rate_increases_with_volume() {
        let cfg = SampleRateConfig::new(2)
            .with_volume_threshold(10)
            .with_max_rate(20);
        let mut sr = SampleRate::new(cfg);
        // Simulate high volume
        let entry = make_entry("info");
        for _ in 0..50 {
            sr.should_emit(&entry);
        }
        let rate = sr.effective_rate();
        assert!(rate > 2, "rate should increase with volume");
        assert!(rate <= 20, "rate should not exceed max");
    }
}
