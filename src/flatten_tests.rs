#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::flatten::{flatten_value, FlattenConfig};

    #[test]
    fn test_flat_object_unchanged() {
        let val = json!({"a": 1, "b": "hello"});
        let cfg = FlattenConfig::default();
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["a"], json!(1));
        assert_eq!(result["b"], json!("hello"));
    }

    #[test]
    fn test_nested_object_flattened() {
        let val = json!({"a": {"b": {"c": 42}}});
        let cfg = FlattenConfig::default();
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["a.b.c"], json!(42));
        assert!(!result.contains_key("a"));
    }

    #[test]
    fn test_custom_separator() {
        let val = json!({"x": {"y": true}});
        let cfg = FlattenConfig::new("_");
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["x_y"], json!(true));
    }

    #[test]
    fn test_max_depth_respected() {
        let val = json!({"a": {"b": {"c": 99}}});
        let cfg = FlattenConfig::new(".").with_max_depth(1);
        let result = flatten_value(&val, &cfg);
        // At depth 1, "a" is expanded but its value {"b":{"c":99}} is kept as-is
        assert_eq!(result["a.b"], json!({"c": 99}));
        assert!(!result.contains_key("a.b.c"));
    }

    #[test]
    fn test_prefix_applied() {
        let val = json!({"level": "info"});
        let cfg = FlattenConfig::new(".").with_prefix("log");
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["log.level"], json!("info"));
    }

    #[test]
    fn test_array_values_kept_as_is() {
        let val = json!({"tags": ["a", "b"]});
        let cfg = FlattenConfig::default();
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["tags"], json!(["a", "b"]));
    }

    #[test]
    fn test_mixed_nested() {
        let val = json!({"meta": {"host": "srv1", "port": 8080}, "msg": "ok"});
        let cfg = FlattenConfig::default();
        let result = flatten_value(&val, &cfg);
        assert_eq!(result["meta.host"], json!("srv1"));
        assert_eq!(result["meta.port"], json!(8080));
        assert_eq!(result["msg"], json!("ok"));
    }

    #[test]
    fn test_empty_object() {
        let val = json!({});
        let cfg = FlattenConfig::default();
        let result = flatten_value(&val, &cfg);
        assert!(result.is_empty());
    }
}
