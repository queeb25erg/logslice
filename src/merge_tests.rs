#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::merge::Merger;
    use crate::merge_config::MergeConfig;
    use std::collections::HashMap;

    fn make_entry(ts: &str, msg: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("timestamp".to_string(), ts.to_string());
        fields.insert("message".to_string(), msg.to_string());
        LogEntry { fields }
    }

    #[test]
    fn test_merge_empty_streams() {
        let merger = Merger::new(MergeConfig::default());
        let result = merger.merge(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_merge_single_stream() {
        let merger = Merger::new(MergeConfig::default());
        let stream = vec![
            make_entry("2024-01-01T10:00:00", "a"),
            make_entry("2024-01-01T10:01:00", "b"),
        ];
        let result = merger.merge(vec![stream]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fields["message"], "a");
        assert_eq!(result[1].fields["message"], "b");
    }

    #[test]
    fn test_merge_two_sorted_streams() {
        let merger = Merger::new(MergeConfig::default());
        let s1 = vec![
            make_entry("2024-01-01T10:00:00", "s1-first"),
            make_entry("2024-01-01T10:02:00", "s1-second"),
        ];
        let s2 = vec![
            make_entry("2024-01-01T10:01:00", "s2-first"),
            make_entry("2024-01-01T10:03:00", "s2-second"),
        ];
        let result = merger.merge(vec![s1, s2]);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].fields["message"], "s1-first");
        assert_eq!(result[1].fields["message"], "s2-first");
        assert_eq!(result[2].fields["message"], "s1-second");
        assert_eq!(result[3].fields["message"], "s2-second");
    }

    #[test]
    fn test_merge_with_empty_stream() {
        let merger = Merger::new(MergeConfig::default());
        let s1 = vec![make_entry("2024-01-01T10:00:00", "only")];
        let result = merger.merge(vec![s1, vec![]]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fields["message"], "only");
    }

    #[test]
    fn test_merge_custom_timestamp_field() {
        let config = MergeConfig::new("ts");
        let merger = Merger::new(config);
        let mut fields1 = HashMap::new();
        fields1.insert("ts".to_string(), "2024-01-01T09:00:00".to_string());
        let mut fields2 = HashMap::new();
        fields2.insert("ts".to_string(), "2024-01-01T08:00:00".to_string());
        let s1 = vec![LogEntry { fields: fields1 }];
        let s2 = vec![LogEntry { fields: fields2 }];
        let result = merger.merge(vec![s1, s2]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].fields["ts"], "2024-01-01T08:00:00");
        assert_eq!(result[1].fields["ts"], "2024-01-01T09:00:00");
    }
}
