#[cfg(test)]
mod tests {
    use super::super::pipeline::Pipeline;
    use super::super::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(ts: i64, level: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("level".to_string(), level.to_string());
        LogEntry {
            timestamp: ts,
            message: format!("msg at {}", ts),
            fields,
            raw: String::new(),
        }
    }

    #[test]
    fn test_pipeline_no_filters_returns_all() {
        let entries = vec![make_entry(100, "info"), make_entry(200, "error")];
        let pipeline = Pipeline::new();
        let result = pipeline.run(entries).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_pipeline_empty_input() {
        let pipeline = Pipeline::new();
        let result = pipeline.run(vec![]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_pipeline_with_limit() {
        let entries = vec![
            make_entry(100, "info"),
            make_entry(200, "info"),
            make_entry(300, "info"),
        ];
        let pipeline = Pipeline::new().with_limit(2);
        let result = pipeline.run(entries).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_pipeline_limit_larger_than_entries() {
        let entries = vec![make_entry(100, "info"), make_entry(200, "info")];
        let pipeline = Pipeline::new().with_limit(10);
        let result = pipeline.run(entries).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_pipeline_default() {
        let pipeline = Pipeline::default();
        assert!(pipeline.time_range.is_none());
        assert!(pipeline.field_chain.is_none());
        assert!(pipeline.limit.is_none());
    }

    #[test]
    fn test_pipeline_preserves_order() {
        let entries = vec![
            make_entry(300, "info"),
            make_entry(100, "info"),
            make_entry(200, "info"),
        ];
        let pipeline = Pipeline::new();
        let result = pipeline.run(entries).unwrap();
        assert_eq!(result[0].timestamp, 300);
        assert_eq!(result[1].timestamp, 100);
        assert_eq!(result[2].timestamp, 200);
    }
}
