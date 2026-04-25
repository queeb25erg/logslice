#[cfg(test)]
mod tests {
    use crate::correlate::{CorrelatedGroup, Correlator};
    use crate::correlate_config::CorrelateConfig;
    use crate::log_entry::LogEntry;
    use serde_json::json;
    use std::collections::HashMap;

    fn make_entry(request_id: &str, msg: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("request_id".to_string(), json!(request_id));
        fields.insert("message".to_string(), json!(msg));
        LogEntry {
            timestamp: chrono::Utc::now(),
            fields,
            raw: format!("{{\"request_id\":\"{}\",\"message\":\"{}\"}}", request_id, msg),
        }
    }

    fn make_entry_no_id(msg: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("message".to_string(), json!(msg));
        LogEntry {
            timestamp: chrono::Utc::now(),
            fields,
            raw: format!("{{\"message\":\"{}\"}}", msg),
        }
    }

    #[test]
    fn test_basic_correlation() {
        let config = CorrelateConfig::new("request_id");
        let correlator = Correlator::new(config);
        let entries = vec![
            make_entry("abc", "start"),
            make_entry("xyz", "other"),
            make_entry("abc", "end"),
        ];
        let groups = correlator.correlate(&entries);
        assert_eq!(groups.len(), 2);
        let abc = groups.iter().find(|g| g.key == "abc").unwrap();
        assert_eq!(abc.entries.len(), 2);
    }

    #[test]
    fn test_min_group_size_filter() {
        let config = CorrelateConfig::new("request_id").min_group_size(2);
        let correlator = Correlator::new(config);
        let entries = vec![
            make_entry("abc", "start"),
            make_entry("abc", "end"),
            make_entry("solo", "alone"),
        ];
        let groups = correlator.correlate(&entries);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].key, "abc");
    }

    #[test]
    fn test_include_unkeyed() {
        let config = CorrelateConfig::new("request_id").include_unkeyed(true);
        let correlator = Correlator::new(config);
        let entries = vec![make_entry("abc", "start"), make_entry_no_id("no id here")];
        let groups = correlator.correlate(&entries);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|g| g.key == "__unkeyed__"));
    }

    #[test]
    fn test_exclude_unkeyed_by_default() {
        let config = CorrelateConfig::new("request_id");
        let correlator = Correlator::new(config);
        let entries = vec![make_entry("abc", "start"), make_entry_no_id("no id here")];
        let groups = correlator.correlate(&entries);
        assert_eq!(groups.len(), 1);
        assert!(!groups.iter().any(|g| g.key == "__unkeyed__"));
    }

    #[test]
    fn test_annotate_key_on_flatten() {
        let config = CorrelateConfig::new("request_id")
            .annotate_key(true)
            .annotation_field("_corr");
        let correlator = Correlator::new(config);
        let entries = vec![make_entry("abc", "start"), make_entry("abc", "end")];
        let groups = correlator.correlate(&entries);
        let flat = correlator.flatten(&groups);
        for entry in &flat {
            assert!(entry.fields.contains_key("_corr"));
            assert_eq!(entry.fields["_corr"], json!("abc"));
        }
    }

    #[test]
    fn test_flatten_without_annotation() {
        let config = CorrelateConfig::new("request_id");
        let correlator = Correlator::new(config);
        let entries = vec![make_entry("abc", "start"), make_entry("abc", "end")];
        let groups = correlator.correlate(&entries);
        let flat = correlator.flatten(&groups);
        assert_eq!(flat.len(), 2);
        assert!(!flat[0].fields.contains_key("_correlation_key"));
    }
}
