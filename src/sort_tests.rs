use std::collections::HashMap;
use chrono::Utc;
use crate::log_entry::LogEntry;
use crate::sort::{sort_entries, sort_by_timestamp, SortConfig, SortOrder};

fn make_entry(ts_offset_secs: i64, fields: Vec<(&str, &str)>) -> LogEntry {
    let mut map = HashMap::new();
    for (k, v) in fields {
        map.insert(k.to_string(), v.to_string());
    }
    LogEntry {
        timestamp: Utc::now() + chrono::Duration::seconds(ts_offset_secs),
        raw: String::new(),
        fields: map,
    }
}

#[test]
fn test_sort_by_field_ascending() {
    let mut entries = vec![
        make_entry(0, vec![("level", "warn")]),
        make_entry(0, vec![("level", "error")]),
        make_entry(0, vec![("level", "info")]),
    ];
    let config = SortConfig::ascending("level");
    sort_entries(&mut entries, &config);
    assert_eq!(entries[0].fields["level"], "error");
    assert_eq!(entries[1].fields["level"], "info");
    assert_eq!(entries[2].fields["level"], "warn");
}

#[test]
fn test_sort_by_field_descending() {
    let mut entries = vec![
        make_entry(0, vec![("level", "info")]),
        make_entry(0, vec![("level", "warn")]),
        make_entry(0, vec![("level", "error")]),
    ];
    let config = SortConfig::descending("level");
    sort_entries(&mut entries, &config);
    assert_eq!(entries[0].fields["level"], "warn");
    assert_eq!(entries[2].fields["level"], "error");
}

#[test]
fn test_sort_by_timestamp_ascending() {
    let mut entries = vec![
        make_entry(10, vec![]),
        make_entry(0, vec![]),
        make_entry(5, vec![]),
    ];
    sort_by_timestamp(&mut entries, &SortOrder::Ascending);
    let ts: Vec<_> = entries.iter().map(|e| e.timestamp).collect();
    assert!(ts[0] <= ts[1] && ts[1] <= ts[2]);
}

#[test]
fn test_sort_by_timestamp_descending() {
    let mut entries = vec![
        make_entry(0, vec![]),
        make_entry(10, vec![]),
        make_entry(5, vec![]),
    ];
    sort_by_timestamp(&mut entries, &SortOrder::Descending);
    let ts: Vec<_> = entries.iter().map(|e| e.timestamp).collect();
    assert!(ts[0] >= ts[1] && ts[1] >= ts[2]);
}

#[test]
fn test_sort_missing_field_treated_as_empty() {
    let mut entries = vec![
        make_entry(0, vec![("level", "info")]),
        make_entry(0, vec![]),
    ];
    let config = SortConfig::ascending("level");
    sort_entries(&mut entries, &config);
    assert_eq!(entries[0].fields.get("level"), None);
}
