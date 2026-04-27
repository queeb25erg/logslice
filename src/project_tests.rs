#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::project::{ProjectConfig, Projector};

    fn make_entry() -> serde_json::Value {
        json!({
            "timestamp": "2024-01-01T00:00:00Z",
            "level": "INFO",
            "message": "hello world",
            "service": "api",
            "debug": "verbose data"
        })
    }

    #[test]
    fn include_keeps_only_specified_fields() {
        let cfg = ProjectConfig::include(vec!["level", "message"]);
        let projector = Projector::new(cfg);
        let result = projector.apply(&make_entry());
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("level"));
        assert!(obj.contains_key("message"));
        assert!(!obj.contains_key("timestamp"));
        assert!(!obj.contains_key("service"));
        assert!(!obj.contains_key("debug"));
    }

    #[test]
    fn exclude_removes_specified_fields() {
        let cfg = ProjectConfig::exclude(vec!["debug", "service"]);
        let projector = Projector::new(cfg);
        let result = projector.apply(&make_entry());
        let obj = result.as_object().unwrap();
        assert!(!obj.contains_key("debug"));
        assert!(!obj.contains_key("service"));
        assert!(obj.contains_key("level"));
        assert!(obj.contains_key("message"));
        assert!(obj.contains_key("timestamp"));
    }

    #[test]
    fn include_with_nonexistent_field_returns_empty() {
        let cfg = ProjectConfig::include(vec!["nonexistent"]);
        let projector = Projector::new(cfg);
        let result = projector.apply(&make_entry());
        let obj = result.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn non_object_value_passes_through_unchanged() {
        let cfg = ProjectConfig::include(vec!["level"]);
        let projector = Projector::new(cfg);
        let scalar = serde_json::Value::String("raw string".to_string());
        let result = projector.apply(&scalar);
        assert_eq!(result, scalar);
    }

    #[test]
    fn config_is_valid_with_fields() {
        let cfg = ProjectConfig::include(vec!["level"]);
        assert!(cfg.is_valid());
    }

    #[test]
    fn config_is_invalid_without_fields() {
        let cfg = ProjectConfig {
            fields: vec![],
            mode: crate::project::ProjectMode::Include,
        };
        assert!(!cfg.is_valid());
    }

    #[test]
    fn projector_exposes_config() {
        let cfg = ProjectConfig::exclude(vec!["debug"]);
        let projector = Projector::new(cfg);
        assert_eq!(projector.config().fields, vec!["debug"]);
    }
}
