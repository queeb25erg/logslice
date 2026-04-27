#[cfg(test)]
mod tests {
    use crate::route_config::RouteConfig;

    #[test]
    fn test_new_has_no_rules() {
        let cfg = RouteConfig::new("default");
        assert!(cfg.rules.is_empty());
        assert_eq!(cfg.default_destination, "default");
    }

    #[test]
    fn test_add_rule() {
        let mut cfg = RouteConfig::new("fallback");
        cfg.add_rule("level", Some("error".to_string()), "errors");
        assert_eq!(cfg.rules.len(), 1);
        assert_eq!(cfg.rules[0].field, "level");
        assert_eq!(cfg.rules[0].pattern, Some("error".to_string()));
        assert_eq!(cfg.rules[0].destination, "errors");
    }

    #[test]
    fn test_with_rule_builder() {
        let cfg = RouteConfig::new("default")
            .with_rule("service", Some("auth".to_string()), "auth-logs")
            .with_rule("level", Some("warn".to_string()), "warnings");
        assert_eq!(cfg.rules.len(), 2);
        assert_eq!(cfg.rules[0].destination, "auth-logs");
        assert_eq!(cfg.rules[1].destination, "warnings");
    }

    #[test]
    fn test_rule_without_pattern() {
        let mut cfg = RouteConfig::new("default");
        cfg.add_rule("trace_id", None, "traced");
        assert!(cfg.rules[0].pattern.is_none());
    }
}
