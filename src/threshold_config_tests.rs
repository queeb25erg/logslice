#[cfg(test)]
mod tests {
    use super::super::threshold_config::{ThresholdConfig, ThresholdOp};

    #[test]
    fn test_default_threshold_config() {
        let cfg = ThresholdConfig::default();
        assert_eq!(cfg.field, "");
        assert_eq!(cfg.value, 0.0);
        assert!(matches!(cfg.op, ThresholdOp::Gt));
    }

    #[test]
    fn test_threshold_config_gt() {
        let cfg = ThresholdConfig {
            field: "latency".to_string(),
            value: 500.0,
            op: ThresholdOp::Gt,
            label: Some("high_latency".to_string()),
        };
        assert!(cfg.matches(600.0));
        assert!(!cfg.matches(500.0));
        assert!(!cfg.matches(400.0));
    }

    #[test]
    fn test_threshold_config_gte() {
        let cfg = ThresholdConfig {
            field: "latency".to_string(),
            value: 500.0,
            op: ThresholdOp::Gte,
            label: None,
        };
        assert!(cfg.matches(500.0));
        assert!(cfg.matches(501.0));
        assert!(!cfg.matches(499.0));
    }

    #[test]
    fn test_threshold_config_lt() {
        let cfg = ThresholdConfig {
            field: "score".to_string(),
            value: 10.0,
            op: ThresholdOp::Lt,
            label: None,
        };
        assert!(cfg.matches(9.9));
        assert!(!cfg.matches(10.0));
    }

    #[test]
    fn test_threshold_config_lte() {
        let cfg = ThresholdConfig {
            field: "score".to_string(),
            value: 10.0,
            op: ThresholdOp::Lte,
            label: None,
        };
        assert!(cfg.matches(10.0));
        assert!(cfg.matches(9.0));
        assert!(!cfg.matches(10.1));
    }

    #[test]
    fn test_threshold_config_eq() {
        let cfg = ThresholdConfig {
            field: "code".to_string(),
            value: 200.0,
            op: ThresholdOp::Eq,
            label: Some("ok".to_string()),
        };
        assert!(cfg.matches(200.0));
        assert!(!cfg.matches(201.0));
    }

    #[test]
    fn test_label_defaults_to_field_name() {
        let cfg = ThresholdConfig {
            field: "cpu".to_string(),
            value: 90.0,
            op: ThresholdOp::Gt,
            label: None,
        };
        assert_eq!(cfg.effective_label(), "cpu");
    }

    #[test]
    fn test_label_uses_custom_when_set() {
        let cfg = ThresholdConfig {
            field: "cpu".to_string(),
            value: 90.0,
            op: ThresholdOp::Gt,
            label: Some("cpu_alert".to_string()),
        };
        assert_eq!(cfg.effective_label(), "cpu_alert");
    }
}
