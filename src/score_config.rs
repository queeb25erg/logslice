use std::collections::HashMap;

/// Configuration for the Scorer.
#[derive(Debug, Clone)]
pub struct ScoreConfig {
    /// Map of field name → numeric weight applied to the field's value.
    pub field_weights: HashMap<String, f64>,

    /// List of (field, keyword, boost) tuples: if `field` contains `keyword`,
    /// add `boost` to the score.
    pub keyword_boosts: Vec<(String, String, f64)>,

    /// Name of the field written back into the entry.
    pub output_field: String,

    /// Optional minimum score; entries below this are dropped by `filter_by_threshold`.
    pub min_score: Option<f64>,
}

impl ScoreConfig {
    pub fn new(output_field: impl Into<String>) -> Self {
        Self {
            field_weights: HashMap::new(),
            keyword_boosts: Vec::new(),
            output_field: output_field.into(),
            min_score: None,
        }
    }

    pub fn with_weight(mut self, field: impl Into<String>, weight: f64) -> Self {
        self.field_weights.insert(field.into(), weight);
        self
    }

    pub fn with_keyword_boost(
        mut self,
        field: impl Into<String>,
        keyword: impl Into<String>,
        boost: f64,
    ) -> Self {
        self.keyword_boosts.push((field.into(), keyword.into(), boost));
        self
    }

    pub fn with_min_score(mut self, min: f64) -> Self {
        self.min_score = Some(min);
        self
    }
}
