use super::*;
use crate::log_entry::LogEntry;
use crate::redact_config::RedactConfig;
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
fn test_redact_single_field() {
    let config = RedactConfig::new(vec!["password".to_string()]);
    let redactor = Redactor::new(config);
    let mut entry = make_entry(&[("user", "alice"), ("password", "secret123")]);
    redactor.redact(&mut entry);
    assert_eq!(entry.fields["password"], "***");
    assert_eq!(entry.fields["user"], "alice");
}

#[test]
fn test_redact_multiple_fields() {
    let config = RedactConfig::new(vec!["token".to_string(), "ssn".to_string()]);
    let redactor = Redactor::new(config);
    let mut entry = make_entry(&[("token", "abc"), ("ssn", "123-45"), ("level", "info")]);
    redactor.redact(&mut entry);
    assert_eq!(entry.fields["token"], "***");
    assert_eq!(entry.fields["ssn"], "***");
    assert_eq!(entry.fields["level"], "info");
}

#[test]
fn test_custom_mask() {
    let config = RedactConfig::new(vec!["email".to_string()]).with_mask("[REDACTED]");
    let redactor = Redactor::new(config);
    let mut entry = make_entry(&[("email", "user@example.com")]);
    redactor.redact(&mut entry);
    assert_eq!(entry.fields["email"], "[REDACTED]");
}

#[test]
fn test_redact_missing_field_is_noop() {
    let config = RedactConfig::new(vec!["secret".to_string()]);
    let redactor = Redactor::new(config);
    let mut entry = make_entry(&[("level", "warn")]);
    redactor.redact(&mut entry);
    assert_eq!(entry.fields["level"], "warn");
    assert!(!entry.fields.contains_key("secret"));
}

#[test]
fn test_redact_all() {
    let config = RedactConfig::new(vec!["key".to_string()]);
    let redactor = Redactor::new(config);
    let entries = vec![
        make_entry(&[("key", "val1")]),
        make_entry(&[("key", "val2"), ("msg", "hello")]),
    ];
    let result = redactor.redact_all(entries);
    assert!(result.iter().all(|e| e.fields.get("key").map(|v| v == "***").unwrap_or(true)));
}

#[test]
fn test_redact_empty_value() {
    // A field that is already empty should still be replaced with the mask.
    let config = RedactConfig::new(vec!["password".to_string()]);
    let redactor = Redactor::new(config);
    let mut entry = make_entry(&[("password", "")]);
    redactor.redact(&mut entry);
    assert_eq!(entry.fields["password"], "***");
}
