#[cfg(test)]
mod tests {
    use crate::mask_config::MaskConfig;

    #[test]
    fn default_mask_is_asterisks() {
        let cfg = MaskConfig::new(vec!["field".to_string()]);
        assert_eq!(cfg.mask, "***");
    }

    #[test]
    fn with_mask_overrides_default() {
        let cfg = MaskConfig::new(vec!["field".to_string()]).with_mask("HIDDEN");
        assert_eq!(cfg.mask, "HIDDEN");
    }

    #[test]
    fn is_empty_true_for_no_fields() {
        let cfg = MaskConfig::default();
        assert!(cfg.is_empty());
    }

    #[test]
    fn is_empty_false_when_fields_set() {
        let cfg = MaskConfig::new(vec!["x".to_string()]);
        assert!(!cfg.is_empty());
    }

    #[test]
    fn default_has_no_fields() {
        let cfg = MaskConfig::default();
        assert!(cfg.fields.is_empty());
    }

    #[test]
    fn clone_produces_equal_config() {
        let cfg = MaskConfig::new(vec!["a".to_string(), "b".to_string()]).with_mask("X");
        let cloned = cfg.clone();
        assert_eq!(cfg, cloned);
    }
}
