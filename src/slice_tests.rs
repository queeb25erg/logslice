#[cfg(test)]
mod tests {
    use crate::field_filter::FieldFilter;
    use crate::field_filter_chain::FieldFilterChain;
    use crate::log_entry::LogEntry;
    use crate::slice::Slicer;
    use crate::time_range::TimeRange;
    use chrono::{TimeZone, Utc};
    use std::collections::HashMap;

    fn entry(ts_secs: Option<i64>, fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            timestamp: ts_secs.map(|s| Utc.timestamp_opt(s, 0).unwrap()),
            raw: String::new(),
            fields: map,
        }
    }

    #[test]
    fn test_no_filters_returns_all() {
        let entries = vec![entry(None, &[]), entry(None, &[])];
        let slicer = Slicer::new(None, FieldFilterChain::new(vec![]));
        assert_eq!(slicer.apply(&entries).len(), 2);
    }

    #[test]
    fn test_time_range_filters_entries() {
        let start = Utc.timestamp_opt(1000, 0).unwrap();
        let end = Utc.timestamp_opt(2000, 0).unwrap();
        let tr = TimeRange::new(Some(start), Some(end));
        let entries = vec![
            entry(Some(500), &[]),
            entry(Some(1500), &[]),
            entry(Some(2500), &[]),
        ];
        let slicer = Slicer::new(Some(tr), FieldFilterChain::new(vec![]));
        let result = slicer.apply(&entries);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_field_filter_applied() {
        let entries = vec![
            entry(None, &[("level", "info")]),
            entry(None, &[("level", "error")]),
        ];
        let f = FieldFilter::new("level".into(), "error".into());
        let slicer = Slicer::new(None, FieldFilterChain::new(vec![f]));
        let result = slicer.apply(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fields.get("level").unwrap(), "error");
    }

    #[test]
    fn test_entry_without_timestamp_excluded_when_range_set() {
        let start = Utc.timestamp_opt(0, 0).unwrap();
        let end = Utc.timestamp_opt(9999, 0).unwrap();
        let tr = TimeRange::new(Some(start), Some(end));
        let entries = vec![entry(None, &[])];
        let slicer = Slicer::new(Some(tr), FieldFilterChain::new(vec![]));
        assert_eq!(slicer.apply(&entries).len(), 0);
    }

    #[test]
    fn test_combined_time_range_and_field_filter() {
        let start = Utc.timestamp_opt(1000, 0).unwrap();
        let end = Utc.timestamp_opt(2000, 0).unwrap();
        let tr = TimeRange::new(Some(start), Some(end));
        let entries = vec![
            entry(Some(1500), &[("level", "info")]),
            entry(Some(1500), &[("level", "error")]),
            entry(Some(500), &[("level", "error")]),
        ];
        let f = FieldFilter::new("level".into(), "error".into());
        let slicer = Slicer::new(Some(tr), FieldFilterChain::new(vec![f]));
        let result = slicer.apply(&entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].fields.get("level").unwrap(), "error");
    }
}
