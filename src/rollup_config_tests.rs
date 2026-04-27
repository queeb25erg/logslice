#[cfg(test)]
mod tests {
    use crate::rollup_config::RollupConfig;

    #[test]
    fn test_default_config() {
        let cfg = RollupConfig::default();
        assert_eq!(cfg.key_field, "level");
        assert_eq!(cfg.default_key, "(unknown)");
        assert!(cfg.top_n.is_none());
        assert!(!cfg.include_first);
    }

    #[test]
    fn test_builder_methods() {
        let cfg = RollupConfig::new("service")
            .with_default_key("none")
            .with_top_n(5)
            .with_include_first(true);
        assert_eq!(cfg.key_field, "service");
        assert_eq!(cfg.default_key, "none");
        assert_eq!(cfg.top_n, Some(5));
        assert!(cfg.include_first);
    }

    #[test]
    fn test_new_sets_key_field() {
        let cfg = RollupConfig::new("host");
        assert_eq!(cfg.key_field, "host");
    }

    #[test]
    fn test_top_n_none_by_default() {
        let cfg = RollupConfig::new("env");
        assert!(cfg.top_n.is_none());
    }
}
