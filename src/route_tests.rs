#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::route::Router;
    use crate::route_config::RouteConfig;
    use std::collections::HashMap;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry { fields: map, raw: String::new() }
    }

    #[test]
    fn test_route_matches_first_rule() {
        let cfg = RouteConfig::new("default")
            .with_rule("level", Some("error".to_string()), "errors")
            .with_rule("level", Some("warn".to_string()), "warnings");
        let router = Router::new(cfg);
        let entry = make_entry(&[("level", "error")]);
        assert_eq!(router.route(&entry), "errors");
    }

    #[test]
    fn test_route_falls_through_to_next_rule() {
        let cfg = RouteConfig::new("default")
            .with_rule("level", Some("error".to_string()), "errors")
            .with_rule("level", Some("warn".to_string()), "warnings");
        let router = Router::new(cfg);
        let entry = make_entry(&[("level", "warn")]);
        assert_eq!(router.route(&entry), "warnings");
    }

    #[test]
    fn test_route_default_when_no_match() {
        let cfg = RouteConfig::new("unmatched")
            .with_rule("level", Some("error".to_string()), "errors");
        let router = Router::new(cfg);
        let entry = make_entry(&[("level", "debug")]);
        assert_eq!(router.route(&entry), "unmatched");
    }

    #[test]
    fn test_route_field_absent_skips_rule() {
        let cfg = RouteConfig::new("fallback")
            .with_rule("service", Some("auth".to_string()), "auth");
        let router = Router::new(cfg);
        let entry = make_entry(&[("level", "info")]);
        assert_eq!(router.route(&entry), "fallback");
    }

    #[test]
    fn test_route_pattern_none_matches_field_presence() {
        let cfg = RouteConfig::new("default")
            .with_rule("trace_id", None, "traced");
        let router = Router::new(cfg);
        let entry = make_entry(&[("trace_id", "abc-123")]);
        assert_eq!(router.route(&entry), "traced");
    }

    #[test]
    fn test_partition_groups_entries() {
        let cfg = RouteConfig::new("other")
            .with_rule("level", Some("error".to_string()), "errors")
            .with_rule("level", Some("info".to_string()), "info");
        let router = Router::new(cfg);
        let entries = vec![
            make_entry(&[("level", "error")]),
            make_entry(&[("level", "info")]),
            make_entry(&[("level", "error")]),
            make_entry(&[("level", "debug")]),
        ];
        let partitioned = router.partition(&entries);
        assert_eq!(partitioned["errors"].len(), 2);
        assert_eq!(partitioned["info"].len(), 1);
        assert_eq!(partitioned["other"].len(), 1);
    }
}
