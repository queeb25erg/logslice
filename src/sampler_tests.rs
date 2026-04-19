#[cfg(test)]
mod tests {
    use crate::sampler::Sampler;
    use crate::sampler_config::SamplerConfig;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(msg: &str) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: None,
            message: msg.to_string(),
            fields: HashMap::new(),
            raw: msg.to_string(),
        }
    }

    fn entries(n: usize) -> Vec<LogEntry> {
        (0..n).map(|i| make_entry(&format!("msg {}", i))).collect()
    }

    #[test]
    fn every_n_keeps_correct_entries() {
        let mut s = Sampler::every_n(3);
        let result = s.apply(entries(9));
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].message, "msg 0");
        assert_eq!(result[1].message, "msg 3");
        assert_eq!(result[2].message, "msg 6");
    }

    #[test]
    fn every_1_keeps_all() {
        let mut s = Sampler::every_n(1);
        let result = s.apply(entries(5));
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn percent_100_keeps_all() {
        let mut s = Sampler::percent(100.0);
        let result = s.apply(entries(20));
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn percent_sampling_is_approximate() {
        let mut s = Sampler::percent(50.0);
        let result = s.apply(entries(1000));
        assert!(result.len() > 300 && result.len() < 700, "expected ~50%, got {}", result.len());
    }

    #[test]
    fn counter_tracks_seen_entries() {
        let mut s = Sampler::every_n(2);
        s.apply(entries(10));
        assert_eq!(s.count(), 10);
    }

    #[test]
    fn config_none_not_active() {
        let c = SamplerConfig::from_args(None, None).unwrap();
        assert!(!c.is_active());
    }

    #[test]
    fn config_conflict_errors() {
        let c = SamplerConfig::from_args(Some(2), Some(50.0));
        assert!(c.is_err());
    }

    #[test]
    fn config_invalid_percent_errors() {
        assert!(SamplerConfig::from_args(None, Some(0.0)).is_err());
        assert!(SamplerConfig::from_args(None, Some(101.0)).is_err());
    }

    #[test]
    fn config_every_n_zero_errors() {
        assert!(SamplerConfig::from_args(Some(0), None).is_err());
    }
}
