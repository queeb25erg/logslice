#[cfg(test)]
mod tests {
    use crate::sample_rate_config::SampleRateConfig;

    #[test]
    fn test_default_config() {
        let cfg = SampleRateConfig::default();
        assert_eq!(cfg.base_rate, 10);
        assert_eq!(cfg.max_rate, 100);
        assert_eq!(cfg.volume_threshold, 1000);
        assert!(cfg.always_emit_severity.is_none());
    }

    #[test]
    fn test_builder_methods() {
        let cfg = SampleRateConfig::new(5)
            .with_max_rate(50)
            .with_volume_threshold(500)
            .with_always_emit_severity("error");
        assert_eq!(cfg.base_rate, 5);
        assert_eq!(cfg.max_rate, 50);
        assert_eq!(cfg.volume_threshold, 500);
        assert_eq!(cfg.always_emit_severity.as_deref(), Some("error"));
    }

    #[test]
    fn test_new_sets_base_rate() {
        let cfg = SampleRateConfig::new(20);
        assert_eq!(cfg.base_rate, 20);
    }
}
