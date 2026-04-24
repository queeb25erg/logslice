#[cfg(test)]
mod tests {
    use crate::window::Window;
    use crate::window_config::WindowConfig;
    use crate::log_entry::LogEntry;
    use std::collections::HashMap;

    fn make_entry(msg: &str) -> LogEntry {
        let mut fields = HashMap::new();
        fields.insert("message".to_string(), msg.to_string());
        LogEntry {
            timestamp: None,
            level: None,
            fields,
            raw: msg.to_string(),
        }
    }

    #[test]
    fn test_tumbling_window_fills_and_emits() {
        let cfg = WindowConfig::tumbling(3);
        let mut w = Window::new(cfg);
        assert!(w.push(make_entry("a")).is_none());
        assert!(w.push(make_entry("b")).is_none());
        let result = w.push(make_entry("c"));
        assert!(result.is_some());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        assert!(w.is_empty());
    }

    #[test]
    fn test_tumbling_window_resets_after_emit() {
        let cfg = WindowConfig::tumbling(2);
        let mut w = Window::new(cfg);
        w.push(make_entry("x"));
        w.push(make_entry("y"));
        assert!(w.is_empty());
        assert!(w.push(make_entry("z")).is_none());
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn test_sliding_window_emits_when_full() {
        let cfg = WindowConfig::sliding(3, 1);
        let mut w = Window::new(cfg);
        assert!(w.slide(make_entry("a")).is_none());
        assert!(w.slide(make_entry("b")).is_none());
        let result = w.slide(make_entry("c"));
        assert!(result.is_some());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_sliding_window_advances_by_step() {
        let cfg = WindowConfig::sliding(3, 1);
        let mut w = Window::new(cfg);
        w.slide(make_entry("a"));
        w.slide(make_entry("b"));
        w.slide(make_entry("c"));
        let result = w.slide(make_entry("d"));
        assert!(result.is_some());
        let entries = result.unwrap();
        assert_eq!(entries[0].raw, "b");
        assert_eq!(entries[2].raw, "d");
    }

    #[test]
    fn test_flush_returns_remaining() {
        let cfg = WindowConfig::tumbling(5);
        let mut w = Window::new(cfg);
        w.push(make_entry("a"));
        w.push(make_entry("b"));
        let flushed = w.flush();
        assert!(flushed.is_some());
        assert_eq!(flushed.unwrap().len(), 2);
        assert!(w.flush().is_none());
    }

    #[test]
    fn test_flush_empty_returns_none() {
        let cfg = WindowConfig::tumbling(3);
        let mut w = Window::new(cfg);
        assert!(w.flush().is_none());
    }
}
