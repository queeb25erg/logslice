#[cfg(test)]
mod tests {
    use crate::field_filter::{FieldFilter, FieldOp};
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry { timestamp: None, raw: String::new(), fields: map }
    }

    #[test]
    fn test_parse_equals() {
        let f = FieldFilter::parse("level=error").unwrap();
        assert_eq!(f.field, "level");
        assert_eq!(f.op, FieldOp::Equals);
        assert_eq!(f.value, "error");
    }

    #[test]
    fn test_parse_contains() {
        let f = FieldFilter::parse("msg~timeout").unwrap();
        assert_eq!(f.op, FieldOp::Contains);
        assert_eq!(f.field, "msg");
    }

    #[test]
    fn test_parse_not_equals() {
        let f = FieldFilter::parse("host!=web01").unwrap();
        assert_eq!(f.op, FieldOp::NotEquals);
        assert_eq!(f.value, "web01");
    }

    #[test]
    fn test_parse_invalid() {
        assert!(FieldFilter::parse("noop").is_none());
    }

    #[test]
    fn test_matches_equals() {
        let f = FieldFilter::new("level", FieldOp::Equals, "error");
        let entry = make_entry(&[("level", "error")]);
        assert!(f.matches(&entry));
    }

    #[test]
    fn test_not_matches_equals() {
        let f = FieldFilter::new("level", FieldOp::Equals, "error");
        let entry = make_entry(&[("level", "info")]);
        assert!(!f.matches(&entry));
    }

    #[test]
    fn test_matches_contains() {
        let f = FieldFilter::new("msg", FieldOp::Contains, "timeout");
        let entry = make_entry(&[("msg", "connection timeout reached")]);
        assert!(f.matches(&entry));
    }

    #[test]
    fn test_matches_not_equals() {
        let f = FieldFilter::new("host", FieldOp::NotEquals, "web01");
        let entry = make_entry(&[("host", "web02")]);
        assert!(f.matches(&entry));
    }

    #[test]
    fn test_missing_field_no_match() {
        let f = FieldFilter::new("level", FieldOp::Equals, "error");
        let entry = make_entry(&[]);
        assert!(!f.matches(&entry));
    }
}
