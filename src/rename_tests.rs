#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::log_entry::LogEntry;
    use crate::rename::Renamer;
    use crate::rename_config::RenameConfig;

    fn make_entry(fields: Vec<(&str, &str)>) -> LogEntry {
        let mut entry = LogEntry::default();
        for (k, v) in fields {
            entry.fields.insert(k.to_string(), v.to_string());
        }
        entry
    }

    #[test]
    fn test_rename_single_field() {
        let config = RenameConfig::from_pairs(vec![("msg", "message")]);
        let renamer = Renamer::new(config);
        let entry = make_entry(vec![("msg", "hello world"), ("level", "info")]);
        let result = renamer.apply(entry);
        assert!(result.fields.contains_key("message"));
        assert!(!result.fields.contains_key("msg"));
        assert_eq!(result.fields["message"], "hello world");
    }

    #[test]
    fn test_rename_multiple_fields() {
        let config = RenameConfig::from_pairs(vec![("ts", "timestamp"), ("lvl", "level")]);
        let renamer = Renamer::new(config);
        let entry = make_entry(vec![("ts", "2024-01-01T00:00:00Z"), ("lvl", "warn")]);
        let result = renamer.apply(entry);
        assert!(result.fields.contains_key("timestamp"));
        assert!(result.fields.contains_key("level"));
        assert!(!result.fields.contains_key("ts"));
        assert!(!result.fields.contains_key("lvl"));
    }

    #[test]
    fn test_rename_missing_field_is_noop() {
        let config = RenameConfig::from_pairs(vec![("nonexistent", "new_name")]);
        let renamer = Renamer::new(config);
        let entry = make_entry(vec![("msg", "test")]);
        let result = renamer.apply(entry);
        assert!(result.fields.contains_key("msg"));
        assert!(!result.fields.contains_key("new_name"));
    }

    #[test]
    fn test_rename_empty_config() {
        let config = RenameConfig::default();
        let renamer = Renamer::new(config);
        let entry = make_entry(vec![("msg", "hello"), ("level", "debug")]);
        let result = renamer.apply(entry);
        assert_eq!(result.fields["msg"], "hello");
        assert_eq!(result.fields["level"], "debug");
    }

    #[test]
    fn test_rename_config_is_empty() {
        let config = RenameConfig::default();
        assert!(config.is_empty());
        let config2 = RenameConfig::from_pairs(vec![("a", "b")]);
        assert!(!config2.is_empty());
    }

    #[test]
    fn test_apply_all() {
        let config = RenameConfig::from_pairs(vec![("msg", "message")]);
        let renamer = Renamer::new(config);
        let entries = vec![
            make_entry(vec![("msg", "first")]),
            make_entry(vec![("msg", "second")]),
        ];
        let results = renamer.apply_all(entries);
        assert_eq!(results.len(), 2);
        assert!(results[0].fields.contains_key("message"));
        assert!(results[1].fields.contains_key("message"));
    }
}
