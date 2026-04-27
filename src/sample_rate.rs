use crate::log_entry::LogEntry;
use crate::sample_rate_config::SampleRateConfig;

/// Dynamically adjusts sampling rate based on log volume or severity.
pub struct SampleRate {
    config: SampleRateConfig,
    seen: u64,
    emitted: u64,
}

impl SampleRate {
    pub fn new(config: SampleRateConfig) -> Self {
        Self {
            config,
            seen: 0,
            emitted: 0,
        }
    }

    /// Returns true if this entry should be included in output.
    pub fn should_emit(&mut self, entry: &LogEntry) -> bool {
        self.seen += 1;

        // Always emit if severity meets the always-emit threshold
        if let Some(ref min_sev) = self.config.always_emit_severity {
            if let Some(ref sev) = entry.fields.get("level") {
                if self.severity_rank(sev) >= self.severity_rank(min_sev) {
                    self.emitted += 1;
                    return true;
                }
            }
        }

        let rate = self.effective_rate();
        let emit = self.seen % rate == 0;
        if emit {
            self.emitted += 1;
        }
        emit
    }

    /// Returns current effective sampling rate (1-in-N).
    pub fn effective_rate(&self) -> u64 {
        if self.seen == 0 {
            return self.config.base_rate;
        }
        // Increase rate if volume is high
        let ratio = self.seen / self.config.volume_threshold.max(1);
        let rate = self.config.base_rate * (1 + ratio);
        rate.min(self.config.max_rate)
    }

    pub fn stats(&self) -> (u64, u64) {
        (self.seen, self.emitted)
    }

    fn severity_rank(&self, sev: &str) -> u8 {
        match sev.to_lowercase().as_str() {
            "trace" => 0,
            "debug" => 1,
            "info" => 2,
            "warn" | "warning" => 3,
            "error" => 4,
            "fatal" | "critical" => 5,
            _ => 2,
        }
    }
}
