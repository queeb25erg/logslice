#[cfg(test)]
mod tests {
    use serde_json::{json, Map, Value};
    use crate::log_entry::LogEntry;
    use crate::group_builder::GroupBuilder;

    fn make_entry(fields: serde_json::Value) -> LogEntry {
        let map: Map<String, Value> = fields
            .as_object()
            .cloned()
            .unwrap_or_default();
        LogEntry {
            raw: String::new(),
            fields: map,
            timestamp: None,
        }
    }

    #[test]
    fn test_group_by_single_field() {
        let entries = vec![
            make_entry(json!({"level": "info"})),
            make_entry(json!({"level": "error"})),
            make_entry(json!({"level": "info"})),
        ];
        let grouper = GroupBuilder::new().field("level").build();
        let groups = grouper.group(&entries);
        assert_eq!(groups["info"].len(), 2);
        assert_eq!(groups["error"].len(), 1);
    }

    #[test]
    fn test_group_by_multiple_fields() {
        let entries = vec![
            make_entry(json!({"service": "auth", "level": "info"})),
            make_entry(json!({"service": "auth", "level": "error"})),
            make_entry(json!({"service": "api",  "level": "info"})),
        ];
        let grouper = GroupBuilder::new().field("service").field("level").build();
        let groups = grouper.group(&entries);
        assert_eq!(groups["auth|info"].len(), 1);
        assert_eq!(groups["auth|error"].len(), 1);
        assert_eq!(groups["api|info"].len(), 1);
    }

    #[test]
    fn test_missing_field_uses_placeholder() {
        let entries = vec![
            make_entry(json!({"level": "info"})),
            make_entry(json!({})),
        ];
        let grouper = GroupBuilder::new().field("level").build();
        let groups = grouper.group(&entries);
        assert!(groups.contains_key("<none>"));
        assert_eq!(groups["<none>"].len(), 1);
    }

    #[test]
    fn test_custom_separator() {
        let entries = vec![
            make_entry(json!({"service": "auth", "level": "info"})),
        ];
        let grouper = GroupBuilder::new()
            .field("service")
            .field("level")
            .separator("::")
            .build();
        let groups = grouper.group(&entries);
        assert!(groups.contains_key("auth::info"));
    }

    #[test]
    fn test_fields_accessor() {
        let grouper = GroupBuilder::new().field("a").field("b").build();
        assert_eq!(grouper.fields(), &["a", "b"]);
    }

    #[test]
    fn test_empty_entries() {
        let grouper = GroupBuilder::new().field("level").build();
        let groups = grouper.group(&[]);
        assert!(groups.is_empty());
    }
}
