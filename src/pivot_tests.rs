#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use crate::pivot::Pivot;
    use crate::pivot_config::PivotConfig;
    use crate::log_entry::LogEntry;

    fn make_entry(level: &str, msg: &str) -> LogEntry {
        LogEntry {
            raw: String::new(),
            fields: json!({ "level": level, "message": msg }),
            timestamp: None,
        }
    }

    #[test]
    fn test_pivot_counts_groups() {
        let entries = vec![
            make_entry("info", "started"),
            make_entry("info", "running"),
            make_entry("error", "failed"),
        ];
        let config = PivotConfig::new("level", "message");
        let pivot = Pivot::new(config);
        let result = pivot.apply(&entries);

        assert_eq!(result.len(), 2);
        let error_row = result.iter().find(|e| e.fields["level"] == "error").unwrap();
        assert_eq!(error_row.fields["count"], json!(1));

        let info_row = result.iter().find(|e| e.fields["level"] == "info").unwrap();
        assert_eq!(info_row.fields["count"], json!(2));
    }

    #[test]
    fn test_pivot_collect_values() {
        let entries = vec![
            make_entry("warn", "disk low"),
            make_entry("warn", "memory low"),
        ];
        let config = PivotConfig::new("level", "message").with_collect_values(true);
        let pivot = Pivot::new(config);
        let result = pivot.apply(&entries);

        assert_eq!(result.len(), 1);
        let row = &result[0];
        let values = row.fields["values"].as_array().unwrap();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&Value::String("disk low".to_string())));
        assert!(values.contains(&Value::String("memory low".to_string())));
    }

    #[test]
    fn test_pivot_missing_key_field() {
        let entries = vec![
            LogEntry {
                raw: String::new(),
                fields: json!({ "message": "hello" }),
                timestamp: None,
            },
        ];
        let config = PivotConfig::new("level", "message");
        let pivot = Pivot::new(config);
        let result = pivot.apply(&entries);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fields["level"], json!("(missing)"));
        assert_eq!(result[0].fields["count"], json!(1));
    }

    #[test]
    fn test_pivot_empty_input() {
        let config = PivotConfig::new("level", "message");
        let pivot = Pivot::new(config);
        let result = pivot.apply(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_pivot_sorted_output() {
        let entries = vec![
            make_entry("warn", "w1"),
            make_entry("debug", "d1"),
            make_entry("error", "e1"),
            make_entry("info", "i1"),
        ];
        let config = PivotConfig::new("level", "message");
        let pivot = Pivot::new(config);
        let result = pivot.apply(&entries);
        let keys: Vec<&str> = result
            .iter()
            .map(|e| e.fields["level"].as_str().unwrap())
            .collect();
        assert_eq!(keys, vec!["debug", "error", "info", "warn"]);
    }
}
