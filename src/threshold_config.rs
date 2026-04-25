/// Configuration for the Threshold filter.
#[derive(Debug, Clone)]
pub struct ThresholdConfig {
    /// The field name to inspect (must be parseable as f64).
    pub field: String,
    /// Comparison operator: ">", ">=", "<", "<=", "==", "!="
    pub operator: String,
    /// The threshold value to compare against.
    pub value: f64,
    /// If true, entries where the field is missing or non-numeric are passed through.
    /// If false (default), they are dropped.
    pub pass_on_missing: bool,
}

impl ThresholdConfig {
    pub fn new(field: impl Into<String>, operator: impl Into<String>, value: f64) -> Self {
        Self {
            field: field.into(),
            operator: operator.into(),
            value,
            pass_on_missing: false,
        }
    }

    pub fn with_pass_on_missing(mut self, pass: bool) -> Self {
        self.pass_on_missing = pass;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let cfg = ThresholdConfig::new("latency_ms", ">", 500.0);
        assert_eq!(cfg.field, "latency_ms");
        assert_eq!(cfg.operator, ">");
        assert!((cfg.value - 500.0).abs() < f64::EPSILON);
        assert!(!cfg.pass_on_missing);
    }

    #[test]
    fn test_with_pass_on_missing() {
        let cfg = ThresholdConfig::new("score", "<", 0.5).with_pass_on_missing(true);
        assert!(cfg.pass_on_missing);
    }
}
