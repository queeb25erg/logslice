#[cfg(test)]
mod tests {
    use crate::classify::Classifier;
    use crate::classify_config::ClassifyConfig;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            raw: String::new(),
            fields: map,
            timestamp: None,
        }
    }

    fn make_classifier() -> Classifier {
        let mut cfg = ClassifyConfig::new("category");
        cfg.add_rule("level", "(?i)error", "high").unwrap();
        cfg.add_rule("level", "(?i)warn", "medium").unwrap();
        cfg.add_rule("level", "(?i)info", "low").unwrap();
        let cfg = cfg.with_default("unknown");
        Classifier::new(cfg)
    }

    #[test]
    fn test_classify_error_level() {
        let c = make_classifier();
        let entry = make_entry(&[("level", "ERROR")]);
        let result = c.classify(entry);
        assert_eq!(result.fields.get("category").map(String::as_str), Some("high"));
    }

    #[test]
    fn test_classify_warn_level() {
        let c = make_classifier();
        let entry = make_entry(&[("level", "WARN")]);
        let result = c.classify(entry);
        assert_eq!(result.fields.get("category").map(String::as_str), Some("medium"));
    }

    #[test]
    fn test_classify_info_level() {
        let c = make_classifier();
        let entry = make_entry(&[("level", "info")]);
        let result = c.classify(entry);
        assert_eq!(result.fields.get("category").map(String::as_str), Some("low"));
    }

    #[test]
    fn test_classify_default_when_no_match() {
        let c = make_classifier();
        let entry = make_entry(&[("level", "DEBUG")]);
        let result = c.classify(entry);
        assert_eq!(result.fields.get("category").map(String::as_str), Some("unknown"));
    }

    #[test]
    fn test_classify_no_matching_field() {
        let c = make_classifier();
        let entry = make_entry(&[("message", "hello")]);
        let result = c.classify(entry);
        assert_eq!(result.fields.get("category").map(String::as_str), Some("unknown"));
    }

    #[test]
    fn test_classify_no_default_leaves_field_absent() {
        let mut cfg = ClassifyConfig::new("category");
        cfg.add_rule("level", "error", "high").unwrap();
        let c = Classifier::new(cfg);
        let entry = make_entry(&[("level", "info")]);
        let result = c.classify(entry);
        assert!(result.fields.get("category").is_none());
    }

    #[test]
    fn test_classify_all() {
        let c = make_classifier();
        let entries = vec![
            make_entry(&[("level", "ERROR")]),
            make_entry(&[("level", "info")]),
        ];
        let results = c.classify_all(entries);
        assert_eq!(results[0].fields["category"], "high");
        assert_eq!(results[1].fields["category"], "low");
    }
}
