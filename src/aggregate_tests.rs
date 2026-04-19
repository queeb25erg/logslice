#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::aggregate::{Aggregator, AggregateConfig, AggregateResult};
    use crate::log_entry::LogEntry;

    fn make_entry(field: &str, value: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert(field.to_string(), value.to_string());
        LogEntry {
            raw: format!("{}: {}", field, value),
            timestamp: None,
            fields,
        }
    }

    #[test]
    fn test_count_by_field() {
        let config = AggregateConfig { group_by: "level".to_string(), count: true };
        let mut agg = Aggregator::new(config);
        agg.process(&make_entry("level", "info"));
        agg.process(&make_entry("level", "error"));
        agg.process(&make_entry("level", "info"));
        let results = agg.results();
        assert_eq!(results[0], AggregateResult { key: "info".to_string(), count: 2 });
        assert_eq!(results[1], AggregateResult { key: "error".to_string(), count: 1 });
    }

    #[test]
    fn test_unknown_field_grouped() {
        let config = AggregateConfig { group_by: "missing".to_string(), count: true };
        let mut agg = Aggregator::new(config);
        agg.process(&make_entry("level", "info"));
        let results = agg.results();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "(unknown)");
        assert_eq!(results[0].count, 1);
    }

    #[test]
    fn test_reset_clears_counts() {
        let config = AggregateConfig { group_by: "level".to_string(), count: true };
        let mut agg = Aggregator::new(config);
        agg.process(&make_entry("level", "warn"));
        agg.reset();
        assert!(agg.results().is_empty());
    }

    #[test]
    fn test_sorted_by_count_desc() {
        let config = AggregateConfig { group_by: "svc".to_string(), count: true };
        let mut agg = Aggregator::new(config);
        for _ in 0..3 { agg.process(&make_entry("svc", "auth")); }
        for _ in 0..5 { agg.process(&make_entry("svc", "api")); }
        agg.process(&make_entry("svc", "db"));
        let results = agg.results();
        assert_eq!(results[0].key, "api");
        assert_eq!(results[0].count, 5);
    }
}
