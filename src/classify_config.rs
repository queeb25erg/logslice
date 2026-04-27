use regex::Regex;

/// A single classification rule: if `field` matches `pattern`, assign `category`.
pub struct ClassifyRule {
    pub field: String,
    pub pattern: Regex,
    pub category: String,
}

/// Configuration for the Classifier.
pub struct ClassifyConfig {
    /// Ordered list of rules; first match wins.
    pub rules: Vec<ClassifyRule>,
    /// Field name to write the resulting category into.
    pub output_field: String,
    /// Category to assign when no rule matches (optional).
    pub default_category: Option<String>,
}

impl ClassifyConfig {
    pub fn new(output_field: impl Into<String>) -> Self {
        Self {
            rules: Vec::new(),
            output_field: output_field.into(),
            default_category: None,
        }
    }

    pub fn add_rule(
        &mut self,
        field: impl Into<String>,
        pattern: &str,
        category: impl Into<String>,
    ) -> Result<(), regex::Error> {
        let re = Regex::new(pattern)?;
        self.rules.push(ClassifyRule {
            field: field.into(),
            pattern: re,
            category: category.into(),
        });
        Ok(())
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_category = Some(default.into());
        self
    }
}
