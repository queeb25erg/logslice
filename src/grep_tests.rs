#[cfg(test)]
mod tests {
    use crate::grep::{Grep, GrepConfig};
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(raw: &str, fields: Vec<(&str, &str)>) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry { raw: raw.to_string(), fields: map, timestamp: None }
    }

    #[test]
    fn test_basic_match() {
        let entry = make_entry("error occurred in module", vec![]);
        let grep = Grep::new(GrepConfig::new("error"));
        assert!(grep.matches(&entry));
    }

    #[test]
    fn test_no_match() {
        let entry = make_entry("info: all good", vec![]);
        let grep = Grep::new(GrepConfig::new("error"));
        assert!(!grep.matches(&entry));
    }

    #[test]
    fn test_case_insensitive() {
        let entry = make_entry("ERROR: disk full", vec![]);
        let grep = Grep::new(GrepConfig::new("error").case_insensitive());
        assert!(grep.matches(&entry));
    }

    #[test]
    fn test_case_sensitive_no_match() {
        let entry = make_entry("ERROR: disk full", vec![]);
        let grep = Grep::new(GrepConfig::new("error"));
        assert!(!grep.matches(&entry));
    }

    #[test]
    fn test_invert() {
        let entry = make_entry("info: all good", vec![]);
        let grep = Grep::new(GrepConfig::new("error").inverted());
        assert!(grep.matches(&entry));
    }

    #[test]
    fn test_field_match() {
        let entry = make_entry("raw line", vec![("msg", "connection refused")]);
        let grep = Grep::new(GrepConfig::new("refused").on_field("msg"));
        assert!(grep.matches(&entry));
    }

    #[test]
    fn test_field_missing() {
        let entry = make_entry("raw line", vec![]);
        let grep = Grep::new(GrepConfig::new("refused").on_field("msg"));
        assert!(!grep.matches(&entry));
    }

    #[test]
    fn test_filter_multiple() {
        let entries = vec![
            make_entry("error: timeout", vec![]),
            make_entry("info: ok", vec![]),
            make_entry("error: crash", vec![]),
        ];
        let grep = Grep::new(GrepConfig::new("error"));
        let result = grep.filter(&entries);
        assert_eq!(result.len(), 2);
    }
}
