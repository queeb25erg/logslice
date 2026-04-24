#[cfg(test)]
mod tests {
    use crate::coalesce::Coalescer;
    use crate::coalesce_config::CoalesceConfig;
    use crate::log_entry::LogEntry;
    use serde_json::{json, Map, Value};

    fn make_entry(fields: serde_json::Map<String, Value>) -> LogEntry {
        LogEntry {
            raw: String::new(),
            fields,
            timestamp: None,
        }
    }

    #[test]
    fn test_picks_first_non_empty() {
        let mut fields = Map::new();
        fields.insert("a".to_string(), json!(null));
        fields.insert("b".to_string(), json!(""));
        fields.insert("c".to_string(), json!("hello"));
        let mut entry = make_entry(fields);

        let cfg = CoalesceConfig::new(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "result",
        );
        let coalescer = Coalescer::new(cfg);
        let matched = coalescer.apply(&mut entry);

        assert!(matched);
        assert_eq!(entry.fields["result"], json!("hello"));
    }

    #[test]
    fn test_fallback_when_all_empty() {
        let mut fields = Map::new();
        fields.insert("a".to_string(), json!(null));
        let mut entry = make_entry(fields);

        let cfg = CoalesceConfig::new(vec!["a".to_string()], "result")
            .with_fallback("unknown");
        let coalescer = Coalescer::new(cfg);
        coalescer.apply(&mut entry);

        assert_eq!(entry.fields["result"], json!("unknown"));
    }

    #[test]
    fn test_remove_sources_after_coalesce() {
        let mut fields = Map::new();
        fields.insert("src1".to_string(), json!("value"));
        fields.insert("src2".to_string(), json!("other"));
        let mut entry = make_entry(fields);

        let cfg = CoalesceConfig::new(
            vec!["src1".to_string(), "src2".to_string()],
            "out",
        )
        .with_remove_sources(true);
        let coalescer = Coalescer::new(cfg);
        coalescer.apply(&mut entry);

        assert_eq!(entry.fields["out"], json!("value"));
        assert!(!entry.fields.contains_key("src1"));
        assert!(!entry.fields.contains_key("src2"));
    }

    #[test]
    fn test_no_match_no_fallback_returns_false() {
        let mut fields = Map::new();
        fields.insert("x".to_string(), json!(null));
        let mut entry = make_entry(fields);

        let cfg = CoalesceConfig::new(vec!["x".to_string()], "out");
        let coalescer = Coalescer::new(cfg);
        let matched = coalescer.apply(&mut entry);

        assert!(!matched);
        assert!(!entry.fields.contains_key("out"));
    }

    #[test]
    fn test_apply_all() {
        let mut entries = vec![
            make_entry({ let mut m = Map::new(); m.insert("a".to_string(), json!("first")); m }),
            make_entry({ let mut m = Map::new(); m.insert("b".to_string(), json!("second")); m }),
        ];
        let cfg = CoalesceConfig::new(
            vec!["a".to_string(), "b".to_string()],
            "result",
        );
        let coalescer = Coalescer::new(cfg);
        coalescer.apply_all(&mut entries);

        assert_eq!(entries[0].fields["result"], json!("first"));
        assert_eq!(entries[1].fields["result"], json!("second"));
    }
}
