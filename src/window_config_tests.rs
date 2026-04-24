#[cfg(test)]
mod tests {
    use crate::window_config::WindowConfig;

    #[test]
    fn test_tumbling_window_config() {
        let cfg = WindowConfig::tumbling(5);
        assert_eq!(cfg.size, 5);
        assert_eq!(cfg.step, 5);
        assert!(!cfg.emit_partial);
    }

    #[test]
    fn test_sliding_window_config() {
        let cfg = WindowConfig::sliding(10, 3);
        assert_eq!(cfg.size, 10);
        assert_eq!(cfg.step, 3);
        assert!(!cfg.emit_partial);
    }

    #[test]
    fn test_with_emit_partial() {
        let cfg = WindowConfig::tumbling(4).with_emit_partial(true);
        assert!(cfg.emit_partial);
    }

    #[test]
    fn test_default_config() {
        let cfg = WindowConfig::default();
        assert_eq!(cfg.size, 10);
        assert_eq!(cfg.step, 10);
    }

    #[test]
    #[should_panic(expected = "step must be greater than zero")]
    fn test_sliding_zero_step_panics() {
        WindowConfig::sliding(5, 0);
    }

    #[test]
    #[should_panic(expected = "step must not exceed window size")]
    fn test_sliding_step_exceeds_size_panics() {
        WindowConfig::sliding(3, 5);
    }
}
