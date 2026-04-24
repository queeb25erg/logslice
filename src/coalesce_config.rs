/// Configuration for the coalesce operation.
/// Picks the first non-empty value from `sources` and writes it to `target`.
#[derive(Debug, Clone, PartialEq)]
pub struct CoalesceConfig {
    /// Ordered list of source field names to check.
    pub sources: Vec<String>,
    /// The field name to write the coalesced value into.
    pub target: String,
    /// If true, remove source fields after coalescing (except if source == target).
    pub remove_sources: bool,
    /// Optional fallback value if no source has a non-empty value.
    pub fallback: Option<String>,
}

impl CoalesceConfig {
    pub fn new(sources: Vec<String>, target: impl Into<String>) -> Self {
        Self {
            sources,
            target: target.into(),
            remove_sources: false,
            fallback: None,
        }
    }

    pub fn with_remove_sources(mut self, remove: bool) -> Self {
        self.remove_sources = remove;
        self
    }

    pub fn with_fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = CoalesceConfig::new(
            vec!["field_a".to_string(), "field_b".to_string()],
            "result",
        );
        assert_eq!(cfg.target, "result");
        assert!(!cfg.remove_sources);
        assert!(cfg.fallback.is_none());
    }

    #[test]
    fn test_builder_methods() {
        let cfg = CoalesceConfig::new(vec!["a".to_string()], "out")
            .with_remove_sources(true)
            .with_fallback("n/a");
        assert!(cfg.remove_sources);
        assert_eq!(cfg.fallback, Some("n/a".to_string()));
    }
}
