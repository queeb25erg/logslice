use crate::log_entry::LogEntry;
use crate::truncate::{truncate_entries, truncate_entry, TruncateConfig};
use std::collections::HashMap;

fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
    let mut map = HashMap::new();
    for (k, v) in fields {
        map.insert(k.to_string(), v.to_string());
    }
    LogEntry {
        timestamp: None,
        fields: map,
        raw: String::new(),
    }
}

#[test]
fn test_truncate_long_field() {
    let mut entry = make_entry(&[("msg", &"a".repeat(300))]);
    let config = TruncateConfig::new(100);
    truncate_entry(&mut entry, &config);
    assert_eq!(entry.fields["msg"].len(), 103); // 100 + "..."
    assert!(entry.fields["msg"].ends_with("..."));
}

#[test]
fn test_no_truncate_short_field() {
    let mut entry = make_entry(&[("msg", "short")]);
    let config = TruncateConfig::new(100);
    truncate_entry(&mut entry, &config);
    assert_eq!(entry.fields["msg"], "short");
}

#[test]
fn test_truncate_specific_fields_only() {
    let mut entry = make_entry(&[("msg", &"a".repeat(200)), ("id", &"b".repeat(200))]);
    let config = TruncateConfig::new(50).with_fields(vec!["msg".to_string()]);
    truncate_entry(&mut entry, &config);
    assert!(entry.fields["msg"].len() <= 53);
    assert_eq!(entry.fields["id"].len(), 200);
}

#[test]
fn test_custom_suffix() {
    let mut entry = make_entry(&[("msg", &"x".repeat(100))]);
    let config = TruncateConfig::new(10).with_suffix("[cut]");
    truncate_entry(&mut entry, &config);
    assert!(entry.fields["msg"].ends_with("[cut]"));
}

#[test]
fn test_truncate_entries_batch() {
    let entries = vec![
        make_entry(&[("msg", &"a".repeat(300))]),
        make_entry(&[("msg", "short")]),
    ];
    let config = TruncateConfig::new(50);
    let result = truncate_entries(entries, &config);
    assert!(result[0].fields["msg"].len() <= 53);
    assert_eq!(result[1].fields["msg"], "short");
}

#[test]
fn test_applies_to() {
    let config = TruncateConfig::new(50).with_fields(vec!["msg".to_string()]);
    assert!(config.applies_to("msg"));
    assert!(!config.applies_to("level"));
}

#[test]
fn test_applies_to_all_when_no_fields() {
    let config = TruncateConfig::new(50);
    assert!(config.applies_to("anything"));
}
