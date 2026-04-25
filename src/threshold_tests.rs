#[cfg(test)]
mod tests {
    use super::super::threshold::Threshold;
    use super::super::threshold_config::{ThresholdConfig, ThresholdOp};
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(field: &str, value: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert(field.to_string(), value.to_string());
        LogEntry {
            raw: format!("{}: {}", field, value),
            fields,
            timestamp: None,
        }
    }

    #[test]
    fn test_matches_gt_true() {
        let t = Threshold::new(ThresholdConfig::new("latency", 100.0, ThresholdOp::Gt));
        let entry = make_entry("latency", "150");
        assert!(t.matches(&entry));
    }

    #[test]
    fn test_matches_gt_false() {
        let t = Threshold::new(ThresholdConfig::new("latency", 100.0, ThresholdOp::Gt));
        let entry = make_entry("latency", "100");
        assert!(!t.matches(&entry));
    }

    #[test]
    fn test_matches_missing_field() {
        let t = Threshold::new(ThresholdConfig::new("latency", 100.0, ThresholdOp::Gt));
        let entry = make_entry("other", "200");
        assert!(!t.matches(&entry));
    }

    #[test]
    fn test_matches_non_numeric_field() {
        let t = Threshold::new(ThresholdConfig::new("status", 200.0, ThresholdOp::Eq));
        let entry = make_entry("status", "ok");
        assert!(!t.matches(&entry));
    }

    #[test]
    fn test_apply_filters_entries() {
        let t = Threshold::new(ThresholdConfig::new("cpu", 80.0, ThresholdOp::Gte));
        let entries = vec![
            make_entry("cpu", "90"),
            make_entry("cpu", "70"),
            make_entry("cpu", "80"),
        ];
        let result = t.apply(entries);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fields["cpu"], "90");
        assert_eq!(result[1].fields["cpu"], "80");
    }

    #[test]
    fn test_annotate_adds_label() {
        let cfg = ThresholdConfig::new("mem", 75.0, ThresholdOp::Gt)
            .with_label("mem_alert");
        let t = Threshold::new(cfg);
        let entries = vec![
            make_entry("mem", "80"),
            make_entry("mem", "60"),
        ];
        let result = t.annotate(entries);
        assert_eq!(result[0].fields.get("threshold"), Some(&"mem_alert".to_string()));
        assert!(result[1].fields.get("threshold").is_none());
    }

    #[test]
    fn test_annotate_uses_field_name_when_no_label() {
        let cfg = ThresholdConfig::new("mem", 75.0, ThresholdOp::Gt);
        let t = Threshold::new(cfg);
        let entries = vec![make_entry("mem", "90")];
        let result = t.annotate(entries);
        assert_eq!(result[0].fields.get("threshold"), Some(&"mem".to_string()));
    }
}
