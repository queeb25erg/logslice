use crate::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub struct GrepConfig {
    pub pattern: String,
    pub case_sensitive: bool,
    pub invert: bool,
    pub field: Option<String>,
}

impl GrepConfig {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            case_sensitive: true,
            invert: false,
            field: None,
        }
    }

    pub fn case_insensitive(mut self) -> Self {
        self.case_sensitive = false;
        self
    }

    pub fn inverted(mut self) -> Self {
        self.invert = true;
        self
    }

    pub fn on_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

pub struct Grep {
    config: GrepConfig,
}

impl Grep {
    pub fn new(config: GrepConfig) -> Self {
        Self { config }
    }

    pub fn matches(&self, entry: &LogEntry) -> bool {
        let haystack = match &self.config.field {
            Some(field) => entry.fields.get(field).map(|v| v.as_str()).unwrap_or("").to_string(),
            None => entry.raw.clone(),
        };

        let matched = if self.config.case_sensitive {
            haystack.contains(&self.config.pattern)
        } else {
            haystack.to_lowercase().contains(&self.config.pattern.to_lowercase())
        };

        if self.config.invert { !matched } else { matched }
    }

    pub fn filter<'a>(&self, entries: &'a [LogEntry]) -> Vec<&'a LogEntry> {
        entries.iter().filter(|e| self.matches(e)).collect()
    }
}
