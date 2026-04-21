use crate::flatten::FlattenConfig;

impl FlattenConfig {
    pub fn new(separator: impl Into<String>) -> Self {
        FlattenConfig {
            separator: separator.into(),
            max_depth: None,
            prefix: None,
        }
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Parse a separator string, supporting named options like "dot", "slash", "underscore".
    pub fn parse_separator(s: &str) -> String {
        match s {
            "dot" => ".".to_string(),
            "slash" => "/".to_string(),
            "underscore" | "_" => "_".to_string(),
            "dash" | "-" => "-".to_string(),
            other => other.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_separator_named() {
        assert_eq!(FlattenConfig::parse_separator("dot"), ".");
        assert_eq!(FlattenConfig::parse_separator("slash"), "/");
        assert_eq!(FlattenConfig::parse_separator("underscore"), "_");
        assert_eq!(FlattenConfig::parse_separator("dash"), "-");
    }

    #[test]
    fn test_parse_separator_literal() {
        assert_eq!(FlattenConfig::parse_separator("::"), "::");
    }

    #[test]
    fn test_builder_chain() {
        let cfg = FlattenConfig::new(".").with_max_depth(2).with_prefix("log");
        assert_eq!(cfg.separator, ".");
        assert_eq!(cfg.max_depth, Some(2));
        assert_eq!(cfg.prefix, Some("log".to_string()));
    }
}
