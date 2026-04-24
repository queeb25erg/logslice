use crate::group_config::GroupConfig;
use crate::group::Grouper;

/// Builder for constructing a [`Grouper`] with a fluent API.
pub struct GroupBuilder {
    fields: Vec<String>,
    separator: Option<String>,
    missing_value: Option<String>,
}

impl GroupBuilder {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            separator: None,
            missing_value: None,
        }
    }

    pub fn field(mut self, name: impl Into<String>) -> Self {
        self.fields.push(name.into());
        self
    }

    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = Some(sep.into());
        self
    }

    pub fn missing_value(mut self, val: impl Into<String>) -> Self {
        self.missing_value = Some(val.into());
        self
    }

    pub fn build(self) -> Grouper {
        let mut cfg = GroupConfig::new(self.fields);
        if let Some(sep) = self.separator {
            cfg = cfg.with_separator(sep);
        }
        if let Some(mv) = self.missing_value {
            cfg = cfg.with_missing_value(mv);
        }
        Grouper::new(cfg)
    }
}

impl Default for GroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}
