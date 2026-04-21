/// Configuration for the Annotator module.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotateConfig {
    /// The field name to write the annotation into.
    pub target_field: String,

    /// Optional source field whose value is used as the annotation base.
    /// If None, `default_value` is used directly.
    pub source_field: Option<String>,

    /// Static default value used when `source_field` is absent or missing.
    pub default_value: Option<String>,

    /// Optional prefix prepended to the derived value.
    pub prefix: Option<String>,

    /// Optional suffix appended to the derived value.
    pub suffix: Option<String>,
}

impl AnnotateConfig {
    pub fn new(target_field: impl Into<String>) -> Self {
        Self {
            target_field: target_field.into(),
            source_field: None,
            default_value: None,
            prefix: None,
            suffix: None,
        }
    }

    pub fn with_source(mut self, field: impl Into<String>) -> Self {
        self.source_field = Some(field.into());
        self
    }

    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
}
