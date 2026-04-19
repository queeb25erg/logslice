#[cfg(test)]
mod tests {
    use crate::log_entry::LogEntry;
    use crate::tail::{tail_entries, tail_from_iter, TailConfig};

    fn make_entry(msg: &str) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: None,
            message: msg.to_string(),
            fields: std::collections::HashMap::new(),
            raw: msg.to_string(),
        }
    }

    #[test]
    fn tail_returns_last_n() {
        let entries: Vec<LogEntry> = (0..10).map(|i| make_entry(&format!("msg {}", i))).collect();
        let config = TailConfig::new(3);
        let result = tail_entries(&entries, &config);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].message, "msg 7");
        assert_eq!(result[2].message, "msg 9");
    }

    #[test]
    fn tail_count_exceeds_length() {
        let entries: Vec<LogEntry> = (0..3).map(|i| make_entry(&format!("msg {}", i))).collect();
        let config = TailConfig::new(10);
        let result = tail_entries(&entries, &config);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn tail_zero_count_returns_empty() {
        let entries: Vec<LogEntry> = (0..5).map(|i| make_entry(&format!("msg {}", i))).collect();
        let config = TailConfig::new(0);
        let result = tail_entries(&entries, &config);
        assert!(result.is_empty());
    }

    #[test]
    fn tail_empty_input() {
        let entries: Vec<LogEntry> = vec![];
        let config = TailConfig::new(5);
        let result = tail_entries(&entries, &config);
        assert!(result.is_empty());
    }

    #[test]
    fn tail_from_iter_last_n() {
        let entries: Vec<LogEntry> = (0..10).map(|i| make_entry(&format!("msg {}", i))).collect();
        let config = TailConfig::new(4);
        let result = tail_from_iter(entries.into_iter(), &config);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].message, "msg 6");
        assert_eq!(result[3].message, "msg 9");
    }

    #[test]
    fn tail_from_iter_zero_count() {
        let entries: Vec<LogEntry> = (0..5).map(|i| make_entry(&format!("msg {}", i))).collect();
        let config = TailConfig::new(0);
        let result = tail_from_iter(entries.into_iter(), &config);
        assert!(result.is_empty());
    }
}
