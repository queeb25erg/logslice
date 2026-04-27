use crate::log_entry::LogEntry;
use crate::score_config::ScoreConfig;

/// Assigns a numeric relevance score to a log entry based on field weights.
pub struct Scorer {
    config: ScoreConfig,
}

impl Scorer {
    pub fn new(config: ScoreConfig) -> Self {
        Self { config }
    }

    /// Compute a score for the given entry. Returns the score value.
    pub fn score(&self, entry: &LogEntry) -> f64 {
        let mut total = 0.0_f64;

        for (field, weight) in &self.config.field_weights {
            if let Some(value) = entry.fields.get(field) {
                let numeric = value
                    .as_str()
                    .and_then(|s| s.parse::<f64>().ok())
                    .or_else(|| value.as_f64())
                    .unwrap_or(0.0);
                total += numeric * weight;
            }
        }

        for (field, keyword, boost) in &self.config.keyword_boosts {
            if let Some(value) = entry.fields.get(field) {
                if value
                    .as_str()
                    .map(|s| s.contains(keyword.as_str()))
                    .unwrap_or(false)
                {
                    total += boost;
                }
            }
        }

        total
    }

    /// Annotate the entry with the computed score under the configured output field.
    pub fn annotate(&self, entry: &mut LogEntry) {
        let s = self.score(entry);
        entry
            .fields
            .insert(self.config.output_field.clone(), serde_json::json!(s));
    }

    /// Filter entries whose score meets the minimum threshold.
    pub fn filter_by_threshold(&self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        let threshold = self.config.min_score.unwrap_or(f64::NEG_INFINITY);
        entries
            .into_iter()
            .filter(|e| self.score(e) >= threshold)
            .collect()
    }
}
