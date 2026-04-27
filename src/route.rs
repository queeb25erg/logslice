use crate::route_config::RouteConfig;
use crate::log_entry::LogEntry;

/// Routes log entries to named output channels based on field matching rules.
pub struct Router {
    config: RouteConfig,
}

impl Router {
    pub fn new(config: RouteConfig) -> Self {
        Self { config }
    }

    /// Returns the name of the first matching route for the given entry,
    /// or the default route name if none match.
    pub fn route(&self, entry: &LogEntry) -> String {
        for rule in &self.config.rules {
            if let Some(value) = entry.fields.get(&rule.field) {
                let matched = match &rule.pattern {
                    Some(pat) => value.contains(pat.as_str()),
                    None => true,
                };
                if matched {
                    return rule.destination.clone();
                }
            }
        }
        self.config.default_destination.clone()
    }

    /// Partitions a slice of entries into a map of destination -> entries.
    pub fn partition<'a>(
        &self,
        entries: &'a [LogEntry],
    ) -> std::collections::HashMap<String, Vec<&'a LogEntry>> {
        let mut map: std::collections::HashMap<String, Vec<&'a LogEntry>> =
            std::collections::HashMap::new();
        for entry in entries {
            let dest = self.route(entry);
            map.entry(dest).or_default().push(entry);
        }
        map
    }
}
