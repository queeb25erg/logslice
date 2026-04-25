use crate::log_entry::LogEntry;
use crate::threshold_config::ThresholdConfig;

/// Filters log entries based on a numeric field exceeding (or falling below) a threshold.
pub struct Threshold {
    config: ThresholdConfig,
}

impl Threshold {
    pub fn new(config: ThresholdConfig) -> Self {
        Self { config }
    }

    /// Returns true if the entry passes the threshold check.
    pub fn apply(&self, entry: &LogEntry) -> bool {
        let raw = match entry.fields.get(&self.config.field) {
            Some(v) => v,
            None => return self.config.pass_on_missing,
        };

        let value: f64 = match raw.parse() {
            Ok(v) => v,
            Err(_) => return self.config.pass_on_missing,
        };

        match self.config.operator.as_str() {
            ">" => value > self.config.value,
            ">=" => value >= self.config.value,
            "<" => value < self.config.value,
            "<=" => value <= self.config.value,
            "=" | "==" => (value - self.config.value).abs() < f64::EPSILON,
            "!=" => (value - self.config.value).abs() >= f64::EPSILON,
            _ => false,
        }
    }

    /// Filters a slice of entries, returning only those that pass.
    pub fn filter(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| self.apply(e)).collect()
    }
}
