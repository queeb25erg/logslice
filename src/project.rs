use std::collections::HashMap;
use serde_json::Value;

/// Projects (selects or excludes) specific fields from a log entry.
#[derive(Debug, Clone)]
pub struct Projector {
    config: ProjectConfig,
}

impl Projector {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    /// Apply projection to a JSON object, returning a new object with only
    /// the desired fields (include mode) or all fields except excluded ones
    /// (exclude mode).
    pub fn apply(&self, entry: &Value) -> Value {
        let obj = match entry.as_object() {
            Some(o) => o,
            None => return entry.clone(),
        };

        let result: HashMap<String, Value> = match self.config.mode {
            ProjectMode::Include => obj
                .iter()
                .filter(|(k, _)| self.config.fields.contains(k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            ProjectMode::Exclude => obj
                .iter()
                .filter(|(k, _)| !self.config.fields.contains(k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        };

        Value::Object(result.into_iter().collect())
    }

    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectMode {
    Include,
    Exclude,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub fields: Vec<String>,
    pub mode: ProjectMode,
}

impl ProjectConfig {
    pub fn include(fields: Vec<&str>) -> Self {
        Self {
            fields: fields.into_iter().map(String::from).collect(),
            mode: ProjectMode::Include,
        }
    }

    pub fn exclude(fields: Vec<&str>) -> Self {
        Self {
            fields: fields.into_iter().map(String::from).collect(),
            mode: ProjectMode::Exclude,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.fields.is_empty()
    }
}
