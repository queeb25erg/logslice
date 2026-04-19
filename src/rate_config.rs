/// Configuration for rate-based filtering of log entries.
#[derive(Debug, Clone)]
pub struct RateConfig {
    /// Maximum number of log entries allowed within the window.
    pub max_count: usize,
    /// Sliding window duration in milliseconds.
    pub window_ms: i64,
}

impl RateConfig {
    pub fn new(max_count: usize, window_ms: i64) -> Self {
        Self { max_count, window_ms }
    }

    /// Convenience: per-second rate limit.
    pub fn per_second(max_count: usize) -> Self {
        Self::new(max_count, 1_000)
    }

    /// Convenience: per-minute rate limit.
    pub fn per_minute(max_count: usize) -> Self {
        Self::new(max_count, 60_000)
    }
}

impl Default for RateConfig {
    fn default() -> Self {
        Self::per_second(100)
    }
}
