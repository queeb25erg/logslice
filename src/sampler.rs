/// Log sampling: keep every Nth entry or a random percentage
use crate::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub enum SampleMode {
    EveryN(usize),
    Percent(f64),
}

#[derive(Debug, Clone)]
pub struct Sampler {
    mode: SampleMode,
    counter: usize,
    seed: u64,
}

impl Sampler {
    pub fn every_n(n: usize) -> Self {
        assert!(n > 0, "N must be greater than zero");
        Sampler { mode: SampleMode::EveryN(n), counter: 0, seed: 0 }
    }

    pub fn percent(pct: f64) -> Self {
        assert!(pct > 0.0 && pct <= 100.0, "Percent must be in (0, 100]");
        Sampler { mode: SampleMode::Percent(pct), counter: 0, seed: 12345 }
    }

    pub fn should_keep(&mut self, entry: &LogEntry) -> bool {
        match self.mode {
            SampleMode::EveryN(n) => {
                let keep = self.counter % n == 0;
                self.counter += 1;
                keep
            }
            SampleMode::Percent(pct) => {
                // Simple LCG pseudo-random for deterministic sampling
                self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let _ = entry;
                let val = (self.seed >> 33) as f64 / (u32::MAX as f64);
                self.counter += 1;
                val * 100.0 < pct
            }
        }
    }

    pub fn apply(&mut self, entries: Vec<LogEntry>) -> Vec<LogEntry> {
        entries.into_iter().filter(|e| self.should_keep(e)).collect()
    }

    pub fn count(&self) -> usize {
        self.counter
    }
}
