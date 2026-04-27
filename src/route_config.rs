/// A single routing rule: if `field` contains `pattern` (or exists when pattern is None),
/// send the entry to `destination`.
#[derive(Debug, Clone)]
pub struct RouteRule {
    pub field: String,
    pub pattern: Option<String>,
    pub destination: String,
}

/// Configuration for the Router.
#[derive(Debug, Clone)]
pub struct RouteConfig {
    pub rules: Vec<RouteRule>,
    pub default_destination: String,
}

impl RouteConfig {
    pub fn new(default_destination: impl Into<String>) -> Self {
        Self {
            rules: Vec::new(),
            default_destination: default_destination.into(),
        }
    }

    pub fn add_rule(
        &mut self,
        field: impl Into<String>,
        pattern: Option<String>,
        destination: impl Into<String>,
    ) {
        self.rules.push(RouteRule {
            field: field.into(),
            pattern,
            destination: destination.into(),
        });
    }

    pub fn with_rule(
        mut self,
        field: impl Into<String>,
        pattern: Option<String>,
        destination: impl Into<String>,
    ) -> Self {
        self.add_rule(field, pattern, destination);
        self
    }
}
