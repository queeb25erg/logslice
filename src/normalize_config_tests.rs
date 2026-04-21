#[cfg(test)]
mod tests {
    use crate::normalize_config::NormalizeConfig;

    #[test]
    fn test_default_config() {
        let cfg = NormalizeConfig::default();
        assert!(cfg.lowercase_keys);
        assert!(cfg.trim_values);
        assert!(!cfg.all_fields);
        assert!(cfg.fields.is_empty());
    }

    #[test]
    fn test_new_with_fields() {
        let cfg = NormalizeConfig::new(vec!["level".into(), "msg".into()]);
        assert_eq!(cfg.fields, vec!["level", "msg"]);
        assert!(cfg.lowercase_keys);
        assert!(cfg.trim_values);
    }

    #[test]
    fn test_all_config() {
        let cfg = NormalizeConfig::all();
        assert!(cfg.all_fields);
    }

    #[test]
    fn test_builder_methods() {
        let cfg = NormalizeConfig::default()
            .with_lowercase_keys(false)
            .with_trim_values(false);
        assert!(!cfg.lowercase_keys);
        assert!(!cfg.trim_values);
    }
}
