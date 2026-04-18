#[cfg(test)]
mod tests {
    use super::super::transform::*;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(fields: Vec<(&str, &str)>) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            raw: String::new(),
            timestamp: None,
            fields: map,
        }
    }

    #[test]
    fn test_rename_field() {
        let mut t = Transformer::new();
        t.add_op(TransformOp::RenameField { from: "msg".into(), to: "message".into() });
        let entry = make_entry(vec![("msg", "hello")]);
        let result = t.apply(entry).unwrap();
        assert_eq!(result.fields.get("message"), Some(&"hello".to_string()));
        assert!(!result.fields.contains_key("msg"));
    }

    #[test]
    fn test_add_field() {
        let mut t = Transformer::new();
        t.add_op(TransformOp::AddField { key: "env".into(), value: "prod".into() });
        let entry = make_entry(vec![]);
        let result = t.apply(entry).unwrap();
        assert_eq!(result.fields.get("env"), Some(&"prod".to_string()));
    }

    #[test]
    fn test_remove_field() {
        let mut t = Transformer::new();
        t.add_op(TransformOp::RemoveField { key: "secret".into() });
        let entry = make_entry(vec![("secret", "abc123"), ("level", "info")]);
        let result = t.apply(entry).unwrap();
        assert!(!result.fields.contains_key("secret"));
        assert!(result.fields.contains_key("level"));
    }

    #[test]
    fn test_mask_field() {
        let mut t = Transformer::new();
        t.add_op(TransformOp::MaskField { key: "token".into(), mask: "***".into() });
        let entry = make_entry(vec![("token", "supersecret")]);
        let result = t.apply(entry).unwrap();
        assert_eq!(result.fields.get("token"), Some(&"***".to_string()));
    }

    #[test]
    fn test_parse_rename() {
        let op = parse_transform_op("rename:old:new").unwrap();
        assert!(matches!(op, TransformOp::RenameField { .. }));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_transform_op("unknown:x").is_err());
    }

    #[test]
    fn test_is_empty() {
        let t = Transformer::new();
        assert!(t.is_empty());
    }
}
