#[cfg(test)]
mod tests {
    use crate::normalize::Normalizer;
    use crate::normalize_config::NormalizeConfig;
    use crate::log_entry::LogEntry;
    use serde_json::{json, Value};
    use std::collections::HashMap;

    fn make_entry(fields: Vec<(&str, Value)>) -> LogEntry {
        LogEntry {
            raw: String::new(),
            timestamp: None,
            fields: fields.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        }
    }

    #[test]
    fn test_trim_string_values() {
        let config = NormalizeConfig::new(vec!["msg".into()]);
        let normalizer = Normalizer::new(config);
        let entry = make_entry(vec![("msg", json!("  hello world  "))]);
        let result = normalizer.normalize(entry);
        assert_eq!(result.fields["msg"], json!("hello world"));
    }

    #[test]
    fn test_lowercase_keys() {
        let config = NormalizeConfig::new(vec!["Level".into()]);
        let normalizer = Normalizer::new(config);
        let entry = make_entry(vec![("Level", json!("INFO"))]);
        let result = normalizer.normalize(entry);
        assert!(result.fields.contains_key("level"));
    }

    #[test]
    fn test_non_targeted_field_unchanged() {
        let config = NormalizeConfig::new(vec!["msg".into()]);
        let normalizer = Normalizer::new(config);
        let entry = make_entry(vec![
            ("msg", json!("  trimmed  ")),
            ("Other", json!("  not trimmed  ")),
        ]);
        let result = normalizer.normalize(entry);
        assert_eq!(result.fields["msg"], json!("trimmed"));
        assert_eq!(result.fields["Other"], json!("  not trimmed  "));
    }

    #[test]
    fn test_all_fields_mode() {
        let config = NormalizeConfig::all();
        let normalizer = Normalizer::new(config);
        let entry = make_entry(vec![
            ("Level", json!("  WARN  ")),
            ("Msg", json!("  something  ")),
        ]);
        let result = normalizer.normalize(entry);
        assert!(result.fields.contains_key("level"));
        assert!(result.fields.contains_key("msg"));
        assert_eq!(result.fields["level"], json!("WARN"));
        assert_eq!(result.fields["msg"], json!("something"));
    }

    #[test]
    fn test_non_string_values_unchanged() {
        let config = NormalizeConfig::all();
        let normalizer = Normalizer::new(config);
        let entry = make_entry(vec![("count", json!(42)), ("active", json!(true))]);
        let result = normalizer.normalize(entry);
        assert_eq!(result.fields["count"], json!(42));
        assert_eq!(result.fields["active"], json!(true));
    }

    #[test]
    fn test_normalize_all_batch() {
        let config = NormalizeConfig::all();
        let normalizer = Normalizer::new(config);
        let entries = vec![
            make_entry(vec![("Msg", json!("  a  "))]),
            make_entry(vec![("Msg", json!("  b  "))]),
        ];
        let results = normalizer.normalize_all(entries);
        assert_eq!(results[0].fields["msg"], json!("a"));
        assert_eq!(results[1].fields["msg"], json!("b"));
    }
}
