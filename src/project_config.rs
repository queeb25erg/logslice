use crate::project::{ProjectConfig, ProjectMode};

/// Builder for constructing a `ProjectConfig` fluently.
#[derive(Debug, Default)]
pub struct ProjectConfigBuilder {
    fields: Vec<String>,
    mode: Option<ProjectMode>,
}

impl ProjectConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, name: &str) -> Self {
        self.fields.push(name.to_string());
        self
    }

    pub fn fields(mut self, names: Vec<&str>) -> Self {
        self.fields
            .extend(names.into_iter().map(String::from));
        self
    }

    pub fn include_mode(mut self) -> Self {
        self.mode = Some(ProjectMode::Include);
        self
    }

    pub fn exclude_mode(mut self) -> Self {
        self.mode = Some(ProjectMode::Exclude);
        self
    }

    pub fn build(self) -> Result<ProjectConfig, String> {
        if self.fields.is_empty() {
            return Err("ProjectConfig requires at least one field".to_string());
        }
        let mode = self.mode.unwrap_or(ProjectMode::Include);
        Ok(ProjectConfig {
            fields: self.fields,
            mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults_to_include() {
        let cfg = ProjectConfigBuilder::new()
            .field("level")
            .build()
            .unwrap();
        assert_eq!(cfg.mode, ProjectMode::Include);
    }

    #[test]
    fn builder_returns_error_when_no_fields() {
        let result = ProjectConfigBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_exclude_mode() {
        let cfg = ProjectConfigBuilder::new()
            .fields(vec!["debug", "trace"])
            .exclude_mode()
            .build()
            .unwrap();
        assert_eq!(cfg.mode, ProjectMode::Exclude);
        assert_eq!(cfg.fields.len(), 2);
    }
}
