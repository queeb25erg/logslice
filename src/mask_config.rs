/// Configuration for the field masking feature.
///
/// Fields listed in `fields` will have their values replaced with `mask`
/// in every processed log entry.
#[derive(Debug, Clone, PartialEq)]
pub struct MaskConfig {
    /// Names of fields whose values should be masked.
    pub fields: Vec<String>,
    /// The string used to replace masked values. Defaults to `"***"`.
    pub mask: String,
}

impl MaskConfig {
    pub fn new(fields: Vec<String>) -> Self {
        Self {
            fields,
            mask: "***".to_string(),
        }
    }

    pub fn with_mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = mask.into();
        self
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl Default for MaskConfig {
    fn default() -> Self {
        Self::new(vec![])
    }
}
