#[cfg(test)]
mod tests {
    use crate::enrich::Enrich;
    use crate::enrich_config::EnrichConfig;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            timestamp: None,
            raw: String::new(),
            fields: map,
        }
    }

    #[test]
    fn test_static_field_injected() {
        let cfg = EnrichConfig::new().with_static_field("env", "staging");
        let enrich = Enrich::new(cfg);
        let entry = make_entry(&[("level", "info")]);
        let result = enrich.apply(entry);
        assert_eq!(result.fields.get("env").map(|s| s.as_str()), Some("staging"));
    }

    #[test]
    fn test_static_field_overwrites_existing() {
        let cfg = EnrichConfig::new().with_static_field("level", "warn");
        let enrich = Enrich::new(cfg);
        let entry = make_entry(&[("level", "info")]);
        let result = enrich.apply(entry);
        assert_eq!(result.fields.get("level").map(|s| s.as_str()), Some("warn"));
    }

    #[test]
    fn test_copy_field() {
        let cfg = EnrichConfig::new().with_copy("request_id", "trace_id");
        let enrich = Enrich::new(cfg);
        let entry = make_entry(&[("request_id", "abc-123")]);
        let result = enrich.apply(entry);
        assert_eq!(result.fields.get("trace_id").map(|s| s.as_str()), Some("abc-123"));
        assert_eq!(result.fields.get("request_id").map(|s| s.as_str()), Some("abc-123"));
    }

    #[test]
    fn test_copy_missing_src_is_noop() {
        let cfg = EnrichConfig::new().with_copy("nonexistent", "dst");
        let enrich = Enrich::new(cfg);
        let entry = make_entry(&[("level", "info")]);
        let result = enrich.apply(entry);
        assert!(!result.fields.contains_key("dst"));
    }

    #[test]
    fn test_rename_field() {
        let cfg = EnrichConfig::new().with_rename("msg", "message");
        let enrich = Enrich::new(cfg);
        let entry = make_entry(&[("msg", "hello world")]);
        let result = enrich.apply(entry);
        assert_eq!(result.fields.get("message").map(|s| s.as_str()), Some("hello world"));
        assert!(!result.fields.contains_key("msg"));
    }

    #[test]
    fn test_apply_all() {
        let cfg = EnrichConfig::new().with_static_field("service", "api");
        let enrich = Enrich::new(cfg);
        let entries = vec![
            make_entry(&[("level", "info")]),
            make_entry(&[("level", "error")]),
        ];
        let results = enrich.apply_all(entries);
        assert_eq!(results.len(), 2);
        for r in &results {
            assert_eq!(r.fields.get("service").map(|s| s.as_str()), Some("api"));
        }
    }
}
