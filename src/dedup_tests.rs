#[cfg(test)]
mod tests {
    use super::super::dedup::{Deduplicator, DedupStrategy};
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(message: &str) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: None,
            message: message.to_string(),
            fields: HashMap::new(),
            raw: message.to_string(),
        }
    }

    fn make_entry_with_field(message: &str, key: &str, value: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert(key.to_string(), value.to_string());
        LogEntry {
            timestamp: None,
            level: None,
            message: message.to_string(),
            fields,
            raw: message.to_string(),
        }
    }

    #[test]
    fn test_exact_message_dedup() {
        let mut d = Deduplicator::new(DedupStrategy::ExactMessage);
        let entries = vec![
            make_entry("hello"),
            make_entry("hello"),
            make_entry("world"),
        ];
        let result = d.filter(entries);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].message, "hello");
        assert_eq!(result[1].message, "world");
    }

    #[test]
    fn test_consecutive_dedup_keeps_non_adjacent() {
        let mut d = Deduplicator::new(DedupStrategy::Consecutive);
        let entries = vec![
            make_entry("a"),
            make_entry("a"),
            make_entry("b"),
            make_entry("a"),
        ];
        let result = d.filter(entries);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_by_field_dedup() {
        let mut d = Deduplicator::new(DedupStrategy::ByField("request_id".to_string()));
        let entries = vec![
            make_entry_with_field("msg1", "request_id", "abc"),
            make_entry_with_field("msg2", "request_id", "abc"),
            make_entry_with_field("msg3", "request_id", "xyz"),
        ];
        let result = d.filter(entries);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_empty_input() {
        let mut d = Deduplicator::new(DedupStrategy::ExactMessage);
        let result = d.filter(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_all_unique() {
        let mut d = Deduplicator::new(DedupStrategy::ExactMessage);
        let entries = vec![make_entry("a"), make_entry("b"), make_entry("c")];
        let result = d.filter(entries);
        assert_eq!(result.len(), 3);
    }
}
