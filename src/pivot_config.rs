use serde::{Deserialize, Serialize};

/// Configuration for the pivot operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PivotConfig {
    /// The field whose values become the grouping key.
    pub key_field: String,
    /// The field whose values are aggregated per key group.
    pub value_field: String,
    /// Whether to include the list of collected values in the output.
    #[serde(default)]
    pub collect_values: bool,
}

impl PivotConfig {
    pub fn new(key_field: impl Into<String>, value_field: impl Into<String>) -> Self {
        Self {
            key_field: key_field.into(),
            value_field: value_field.into(),
            collect_values: false,
        }
    }

    pub fn with_collect_values(mut self, collect: bool) -> Self {
        self.collect_values = collect;
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.key_field.is_empty() && !self.value_field.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_collect_values_is_false() {
        let cfg = PivotConfig::new("level", "message");
        assert!(!cfg.collect_values);
    }

    #[test]
    fn test_with_collect_values() {
        let cfg = PivotConfig::new("level", "message").with_collect_values(true);
        assert!(cfg.collect_values);
    }

    #[test]
    fn test_is_valid() {
        assert!(PivotConfig::new("level", "msg").is_valid());
        assert!(!PivotConfig::new("", "msg").is_valid());
        assert!(!PivotConfig::new("level", "").is_valid());
    }
}
