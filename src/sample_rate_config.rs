/// Configuration for adaptive sample rate.
#[derive(Debug, Clone)]
pub struct SampleRateConfig {
    /// Base sampling rate: emit 1 out of every N entries.
    pub base_rate: u64,
    /// Maximum rate cap to avoid dropping too many entries.
    pub max_rate: u64,
    /// Number of entries seen before increasing the rate.
    pub volume_threshold: u64,
    /// Severity level at or above which entries are always emitted.
    pub always_emit_severity: Option<String>,
}

impl SampleRateConfig {
    pub fn new(base_rate: u64) -> Self {
        Self {
            base_rate,
            max_rate: 100,
            volume_threshold: 1000,
            always_emit_severity: None,
        }
    }

    pub fn with_max_rate(mut self, max_rate: u64) -> Self {
        self.max_rate = max_rate;
        self
    }

    pub fn with_volume_threshold(mut self, threshold: u64) -> Self {
        self.volume_threshold = threshold;
        self
    }

    pub fn with_always_emit_severity(mut self, severity: impl Into<String>) -> Self {
        self.always_emit_severity = Some(severity.into());
        self
    }
}

impl Default for SampleRateConfig {
    fn default() -> Self {
        Self::new(10)
    }
}
