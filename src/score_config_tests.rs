#[cfg(test)]
mod tests {
    use crate::score_config::ScoreConfig;

    #[test]
    fn test_default_output_field() {
        let cfg = ScoreConfig::new("relevance");
        assert_eq!(cfg.output_field, "relevance");
    }

    #[test]
    fn test_with_weight_stores_entry() {
        let cfg = ScoreConfig::new("_score").with_weight("priority", 2.5);
        assert_eq!(cfg.field_weights.get("priority"), Some(&2.5));
    }

    #[test]
    fn test_multiple_weights() {
        let cfg = ScoreConfig::new("_score")
            .with_weight("a", 1.0)
            .with_weight("b", 3.0);
        assert_eq!(cfg.field_weights.len(), 2);
        assert_eq!(cfg.field_weights["b"], 3.0);
    }

    #[test]
    fn test_keyword_boost_stored() {
        let cfg = ScoreConfig::new("_score")
            .with_keyword_boost("msg", "fail", 20.0);
        assert_eq!(cfg.keyword_boosts.len(), 1);
        let (f, k, b) = &cfg.keyword_boosts[0];
        assert_eq!(f, "msg");
        assert_eq!(k, "fail");
        assert!((b - 20.0).abs() < 1e-9);
    }

    #[test]
    fn test_min_score_none_by_default() {
        let cfg = ScoreConfig::new("_score");
        assert!(cfg.min_score.is_none());
    }

    #[test]
    fn test_with_min_score() {
        let cfg = ScoreConfig::new("_score").with_min_score(15.0);
        assert_eq!(cfg.min_score, Some(15.0));
    }

    #[test]
    fn test_builder_is_chainable() {
        let cfg = ScoreConfig::new("_score")
            .with_weight("x", 1.0)
            .with_keyword_boost("y", "z", 5.0)
            .with_min_score(3.0);
        assert_eq!(cfg.field_weights.len(), 1);
        assert_eq!(cfg.keyword_boosts.len(), 1);
        assert_eq!(cfg.min_score, Some(3.0));
    }
}
