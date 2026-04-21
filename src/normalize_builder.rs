use crate::normalize::Normalizer;
use crate::normalize_config::NormalizeConfig;

/// Builder for constructing a [`Normalizer`] with a fluent API.
pub struct NormalizeBuilder {
    config: NormalizeConfig,
}

impl NormalizeBuilder {
    pub fn new() -> Self {
        Self {
            config: NormalizeConfig::default(),
        }
    }

    /// Specify explicit fields to normalize.
    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.config.fields = fields.into_iter().map(String::from).collect();
        self
    }

    /// Normalize all fields in each log entry.
    pub fn all_fields(mut self) -> Self {
        self.config.all_fields = true;
        self
    }

    /// Control whether field keys are lowercased.
    pub fn lowercase_keys(mut self, v: bool) -> Self {
        self.config.lowercase_keys = v;
        self
    }

    /// Control whether string values are trimmed.
    pub fn trim_values(mut self, v: bool) -> Self {
        self.config.trim_values = v;
        self
    }

    /// Build the final [`Normalizer`].
    pub fn build(self) -> Normalizer {
        Normalizer::new(self.config)
    }
}

impl Default for NormalizeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
