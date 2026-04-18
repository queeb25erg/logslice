#[cfg(test)]
mod tests {
    use crate::field_filter::FieldFilter;
    use crate::field_filter_chain::FieldFilterChain;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry {
            timestamp: None,
            raw: String::new(),
            fields: map,
        }
    }

    #[test]
    fn test_empty_chain_matches_all() {
        let chain = FieldFilterChain::new(vec![]);
        let entry = make_entry(&[("level", "info")]);
        assert!(chain.matches(&entry));
    }

    #[test]
    fn test_single_filter_match() {
        let f = FieldFilter::new("level".into(), "error".into());
        let chain = FieldFilterChain::new(vec![f]);
        let entry = make_entry(&[("level", "error")]);
        assert!(chain.matches(&entry));
    }

    #[test]
    fn test_single_filter_no_match() {
        let f = FieldFilter::new("level".into(), "error".into());
        let chain = FieldFilterChain::new(vec![f]);
        let entry = make_entry(&[("level", "info")]);
        assert!(!chain.matches(&entry));
    }

    #[test]
    fn test_multiple_filters_all_match() {
        let f1 = FieldFilter::new("level".into(), "warn".into());
        let f2 = FieldFilter::new("service".into(), "auth".into());
        let chain = FieldFilterChain::new(vec![f1, f2]);
        let entry = make_entry(&[("level", "warn"), ("service", "auth")]);
        assert!(chain.matches(&entry));
    }

    #[test]
    fn test_multiple_filters_partial_match() {
        let f1 = FieldFilter::new("level".into(), "warn".into());
        let f2 = FieldFilter::new("service".into(), "auth".into());
        let chain = FieldFilterChain::new(vec![f1, f2]);
        let entry = make_entry(&[("level", "warn"), ("service", "payments")]);
        assert!(!chain.matches(&entry));
    }

    #[test]
    fn test_missing_field_no_match() {
        let f = FieldFilter::new("host".into(), "web01".into());
        let chain = FieldFilterChain::new(vec![f]);
        let entry = make_entry(&[("level", "info")]);
        assert!(!chain.matches(&entry));
    }
}
