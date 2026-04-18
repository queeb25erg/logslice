use crate::dedup::DedupStrategy;

/// Configuration for deduplication parsed from CLI arguments
#[derive(Debug, Clone)]
pub struct DedupConfig {
    pub enabled: bool,
    pub strategy: DedupStrategy,
}

impl Default for DedupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: DedupStrategy::ExactMessage,
        }
    }
}

impl DedupConfig {
    /// Parse dedup config from CLI flag strings
    /// --dedup             => ExactMessage
    /// --dedup=consecutive => Consecutive
    /// --dedup=field:name  => ByField("name")
    pub fn from_flag(flag: Option<&str>) -> Self {
        match flag {
            None => Self::default(),
            Some(value) => {
                let strategy = if value.is_empty() || value == "exact" {
                    DedupStrategy::ExactMessage
                } else if value == "consecutive" {
                    DedupStrategy::Consecutive
                } else if let Some(field) = value.strip_prefix("field:") {
                    DedupStrategy::ByField(field.to_string())
                } else {
                    DedupStrategy::ExactMessage
                };
                Self {
                    enabled: true,
                    strategy,
                }
            }
        }
    }
}
