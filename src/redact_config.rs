/// Configuration for field redaction.
#[derive(Debug, Clone)]
pub struct RedactConfig {
    /// Fields whose values should be masked.
    pub fields: Vec<String>,
    /// The string to replace field values with. Defaults to `"***"`.
    pub mask: String,
}

impl RedactConfig {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            mask: String::from("***"),
        }
    }

    pub fn with_mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = mask.into();
        self
    }

    /// Returns `true` if the given field name should be redacted.
    pub fn should_redact(&self, field: &str) -> bool {
        self.fields.iter().any(|f| f == field)
    }
}

impl Default for RedactConfig {
    fn default() -> Self {
        Self::new(vec![])
    }
}
