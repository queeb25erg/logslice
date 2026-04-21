#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::log_entry::LogEntry;
    use crate::mask::Masker;
    use crate::mask_config::MaskConfig;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
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
    fn masks_specified_field() {
        let config = MaskConfig::new(vec!["password".to_string()]);
        let masker = Masker::new(config);
        let entry = make_entry(&[("user", "alice"), ("password", "s3cr3t")]);
        let result = masker.apply(entry);
        assert_eq!(result.fields["password"], "***");
        assert_eq!(result.fields["user"], "alice");
    }

    #[test]
    fn uses_custom_mask_string() {
        let config = MaskConfig::new(vec!["token".to_string()]).with_mask("[REDACTED]");
        let masker = Masker::new(config);
        let entry = make_entry(&[("token", "abc123")]);
        let result = masker.apply(entry);
        assert_eq!(result.fields["token"], "[REDACTED]");
    }

    #[test]
    fn ignores_absent_fields() {
        let config = MaskConfig::new(vec!["secret".to_string()]);
        let masker = Masker::new(config);
        let entry = make_entry(&[("level", "info")]);
        let result = masker.apply(entry);
        assert!(!result.fields.contains_key("secret"));
        assert_eq!(result.fields["level"], "info");
    }

    #[test]
    fn masks_multiple_fields() {
        let config = MaskConfig::new(vec!["email".to_string(), "ssn".to_string()]);
        let masker = Masker::new(config);
        let entry = make_entry(&[("email", "a@b.com"), ("ssn", "123-45-6789"), ("id", "7")]);
        let result = masker.apply(entry);
        assert_eq!(result.fields["email"], "***");
        assert_eq!(result.fields["ssn"], "***");
        assert_eq!(result.fields["id"], "7");
    }

    #[test]
    fn apply_all_processes_every_entry() {
        let config = MaskConfig::new(vec!["key".to_string()]);
        let masker = Masker::new(config);
        let entries = vec![
            make_entry(&[("key", "val1")]),
            make_entry(&[("key", "val2")]),
        ];
        let results = masker.apply_all(entries);
        assert!(results.iter().all(|e| e.fields["key"] == "***"));
    }

    #[test]
    fn is_active_returns_false_for_empty_config() {
        let masker = Masker::new(MaskConfig::default());
        assert!(!masker.is_active());
    }

    #[test]
    fn is_active_returns_true_when_fields_present() {
        let config = MaskConfig::new(vec!["pw".to_string()]);
        let masker = Masker::new(config);
        assert!(masker.is_active());
    }
}
