#[cfg(test)]
mod tests {
    use super::super::transform_config::TransformConfig;

    #[test]
    fn test_empty_config() {
        let cfg = TransformConfig::default();
        assert!(cfg.is_empty());
        let t = cfg.build_transformer().unwrap();
        assert!(t.is_empty());
    }

    #[test]
    fn test_build_transformer_from_config() {
        let cfg = TransformConfig::new(vec![
            "add:env:staging".to_string(),
            "mask:password:***".to_string(),
        ]);
        assert!(!cfg.is_empty());
        let t = cfg.build_transformer().unwrap();
        assert!(!t.is_empty());
    }

    #[test]
    fn test_invalid_op_returns_error() {
        let cfg = TransformConfig::new(vec!["bogus".to_string()]);
        assert!(cfg.build_transformer().is_err());
    }

    #[test]
    fn test_multiple_ops_order_preserved() {
        let ops = vec![
            "rename:msg:message".to_string(),
            "remove:debug".to_string(),
        ];
        let cfg = TransformConfig::new(ops.clone());
        assert_eq!(cfg.ops, ops);
    }
}
