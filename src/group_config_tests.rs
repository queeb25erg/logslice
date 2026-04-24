#[cfg(test)]
mod tests {
    use crate::group_config::GroupConfig;

    #[test]
    fn test_default_separator_and_missing() {
        let cfg = GroupConfig::new(vec!["level".into()]);
        assert_eq!(cfg.separator, "|");
        assert_eq!(cfg.missing_value, "<none>");
    }

    #[test]
    fn test_with_separator() {
        let cfg = GroupConfig::new(vec!["level".into()]).with_separator("::");
        assert_eq!(cfg.separator, "::");
    }

    #[test]
    fn test_with_missing_value() {
        let cfg = GroupConfig::new(vec!["level".into()]).with_missing_value("N/A");
        assert_eq!(cfg.missing_value, "N/A");
    }

    #[test]
    fn test_multiple_fields() {
        let cfg = GroupConfig::new(vec!["service".into(), "level".into()]);
        assert_eq!(cfg.fields.len(), 2);
        assert_eq!(cfg.fields[0], "service");
        assert_eq!(cfg.fields[1], "level");
    }

    #[test]
    fn test_default_is_empty_fields() {
        let cfg = GroupConfig::default();
        assert!(cfg.fields.is_empty());
    }
}
