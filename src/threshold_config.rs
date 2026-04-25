/// Comparison operator for threshold evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ThresholdOp {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
}

/// Configuration for a numeric field threshold check.
#[derive(Debug, Clone)]
pub struct ThresholdConfig {
    /// The log entry field to evaluate.
    pub field: String,
    /// The threshold value to compare against.
    pub value: f64,
    /// The comparison operator.
    pub op: ThresholdOp,
    /// Optional label to attach when the threshold is breached.
    pub label: Option<String>,
}

impl ThresholdConfig {
    pub fn new(field: impl Into<String>, value: f64, op: ThresholdOp) -> Self {
        Self {
            field: field.into(),
            value,
            op,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns true if `n` satisfies the threshold condition.
    pub fn matches(&self, n: f64) -> bool {
        match self.op {
            ThresholdOp::Gt => n > self.value,
            ThresholdOp::Gte => n >= self.value,
            ThresholdOp::Lt => n < self.value,
            ThresholdOp::Lte => n <= self.value,
            ThresholdOp::Eq => (n - self.value).abs() < f64::EPSILON,
        }
    }

    /// Returns the label to use when annotating a breached entry.
    pub fn effective_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.field)
    }
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            field: String::new(),
            value: 0.0,
            op: ThresholdOp::Gt,
            label: None,
        }
    }
}
