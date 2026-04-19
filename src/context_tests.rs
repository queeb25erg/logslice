#[cfg(test)]
mod tests {
    use crate::context::{apply_context, ContextConfig};
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(msg: &str) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: None,
            message: msg.to_string(),
            fields: HashMap::new(),
            raw: msg.to_string(),
        }
    }

    fn entries(msgs: &[&str]) -> Vec<LogEntry> {
        msgs.iter().map(|m| make_entry(m)).collect()
    }

    #[test]
    fn test_no_matches_returns_empty() {
        let e = entries(&["a", "b", "c"]);
        let result = apply_context(&e, &[], &ContextConfig::new(1, 1));
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_match_no_context() {
        let e = entries(&["a", "b", "c"]);
        let result = apply_context(&e, &[1], &ContextConfig::new(0, 0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].message, "b");
    }

    #[test]
    fn test_before_context() {
        let e = entries(&["a", "b", "c", "d"]);
        let result = apply_context(&e, &[2], &ContextConfig::new(2, 0));
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].message, "a");
        assert_eq!(result[2].message, "c");
    }

    #[test]
    fn test_after_context() {
        let e = entries(&["a", "b", "c", "d"]);
        let result = apply_context(&e, &[1], &ContextConfig::new(0, 2));
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].message, "b");
        assert_eq!(result[2].message, "d");
    }

    #[test]
    fn test_overlapping_contexts_deduped() {
        let e = entries(&["a", "b", "c", "d", "e"]);
        let result = apply_context(&e, &[1, 3], &ContextConfig::new(1, 1));
        // indices: 0,1,2 and 2,3,4 => 0,1,2,3,4
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_boundary_clamping() {
        let e = entries(&["a", "b"]);
        let result = apply_context(&e, &[0], &ContextConfig::new(5, 5));
        assert_eq!(result.len(), 2);
    }
}
