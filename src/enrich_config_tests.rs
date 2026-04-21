#[cfg(test)]
mod tests {
    use crate::enrich_config::EnrichConfig;

    #[test]
    fn test_default_config_is_empty() {
        let cfg = EnrichConfig::default();
        assert!(cfg.static_fields.is_empty());
        assert!(cfg.copy_fields.is_none());
        assert!(cfg.rename_fields.is_none());
    }

    #[test]
    fn test_with_static_field() {
        let cfg = EnrichConfig::new().with_static_field("env", "production");
        assert_eq!(cfg.static_fields.get("env").map(|s| s.as_str()), Some("production"));
    }

    #[test]
    fn test_with_copy() {
        let cfg = EnrichConfig::new().with_copy("request_id", "trace_id");
        let copy = cfg.copy_fields.unwrap();
        assert_eq!(copy.get("request_id").map(|s| s.as_str()), Some("trace_id"));
    }

    #[test]
    fn test_with_rename() {
        let cfg = EnrichConfig::new().with_rename("msg", "message");
        let rename = cfg.rename_fields.unwrap();
        assert_eq!(rename.get("msg").map(|s| s.as_str()), Some("message"));
    }

    #[test]
    fn test_chaining() {
        let cfg = EnrichConfig::new()
            .with_static_field("service", "api")
            .with_static_field("version", "1.0")
            .with_copy("user_id", "actor")
            .with_rename("lvl", "level");
        assert_eq!(cfg.static_fields.len(), 2);
        assert!(cfg.copy_fields.is_some());
        assert!(cfg.rename_fields.is_some());
    }
}
