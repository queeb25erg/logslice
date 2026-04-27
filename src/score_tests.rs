#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::score::Scorer;
    use crate::score_config::ScoreConfig;
    use serde_json::json;

    fn make_entry(fields: serde_json::Value) -> LogEntry {
        LogEntry {
            raw: String::new(),
            timestamp: None,
            fields: fields
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }

    #[test]
    fn test_score_field_weight() {
        let config = ScoreConfig::new("_score").with_weight("priority", 2.0);
        let scorer = Scorer::new(config);
        let entry = make_entry(json!({ "priority": "5" }));
        let s = scorer.score(&entry);
        assert!((s - 10.0).abs() < 1e-9, "expected 10.0, got {s}");
    }

    #[test]
    fn test_score_missing_field_is_zero() {
        let config = ScoreConfig::new("_score").with_weight("missing", 3.0);
        let scorer = Scorer::new(config);
        let entry = make_entry(json!({ "other": "1" }));
        assert_eq!(scorer.score(&entry), 0.0);
    }

    #[test]
    fn test_keyword_boost() {
        let config = ScoreConfig::new("_score")
            .with_keyword_boost("message", "error", 50.0);
        let scorer = Scorer::new(config);
        let entry = make_entry(json!({ "message": "an error occurred" }));
        assert!((scorer.score(&entry) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_keyword_boost_no_match() {
        let config = ScoreConfig::new("_score")
            .with_keyword_boost("message", "error", 50.0);
        let scorer = Scorer::new(config);
        let entry = make_entry(json!({ "message": "all good" }));
        assert_eq!(scorer.score(&entry), 0.0);
    }

    #[test]
    fn test_annotate_writes_score_field() {
        let config = ScoreConfig::new("_score").with_weight("val", 1.0);
        let scorer = Scorer::new(config);
        let mut entry = make_entry(json!({ "val": "7" }));
        scorer.annotate(&mut entry);
        let written = entry.fields.get("_score").unwrap().as_f64().unwrap();
        assert!((written - 7.0).abs() < 1e-9);
    }

    #[test]
    fn test_filter_by_threshold_keeps_above() {
        let config = ScoreConfig::new("_score")
            .with_weight("val", 1.0)
            .with_min_score(5.0);
        let scorer = Scorer::new(config);
        let entries = vec![
            make_entry(json!({ "val": "3" })),
            make_entry(json!({ "val": "6" })),
            make_entry(json!({ "val": "10" })),
        ];
        let result = scorer.filter_by_threshold(entries);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_combined_weight_and_boost() {
        let config = ScoreConfig::new("_score")
            .with_weight("severity", 3.0)
            .with_keyword_boost("level", "CRITICAL", 100.0);
        let scorer = Scorer::new(config);
        let entry = make_entry(json!({ "severity": "4", "level": "CRITICAL" }));
        let s = scorer.score(&entry);
        assert!((s - 112.0).abs() < 1e-9, "expected 112.0, got {s}");
    }
}
