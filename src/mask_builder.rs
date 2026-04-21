use crate::mask_config::MaskConfig;
use crate::mask::Masker;

/// Fluent builder for constructing a [`Masker`] from CLI arguments or
/// configuration sources.
#[derive(Debug, Default)]
pub struct MaskBuilder {
    fields: Vec<String>,
    mask: Option<String>,
}

impl MaskBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single field name to the mask list.
    pub fn field(mut self, name: impl Into<String>) -> Self {
        self.fields.push(name.into());
        self
    }

    /// Add multiple field names at once.
    pub fn fields(mut self, names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields.extend(names.into_iter().map(|n| n.into()));
        self
    }

    /// Override the default mask string.
    pub fn mask_string(mut self, mask: impl Into<String>) -> Self {
        self.mask = Some(mask.into());
        self
    }

    /// Build the [`Masker`].
    pub fn build(self) -> Masker {
        let mut config = MaskConfig::new(self.fields);
        if let Some(m) = self.mask {
            config = config.with_mask(m);
        }
        Masker::new(config)
    }

    /// Build the underlying [`MaskConfig`] without constructing a masker.
    pub fn build_config(self) -> MaskConfig {
        let mut config = MaskConfig::new(self.fields);
        if let Some(m) = self.mask {
            config = config.with_mask(m);
        }
        config
    }
}
