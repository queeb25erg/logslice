/// Configuration and logic for limiting the number of log entries output.

#[derive(Debug, Clone)]
pub struct LimitConfig {
    pub max_entries: usize,
    pub offset: usize,
}

impl LimitConfig {
    pub fn new(max_entries: usize, offset: usize) -> Self {
        Self { max_entries, offset }
    }

    pub fn unlimited() -> Self {
        Self { max_entries: usize::MAX, offset: 0 }
    }
}

pub struct Limiter {
    config: LimitConfig,
    seen: usize,
    emitted: usize,
}

impl Limiter {
    pub fn new(config: LimitConfig) -> Self {
        Self { config, seen: 0, emitted: 0 }
    }

    /// Returns true if the entry should be included in output.
    pub fn accept(&mut self) -> bool {
        let idx = self.seen;
        self.seen += 1;

        if idx < self.config.offset {
            return false;
        }

        if self.emitted >= self.config.max_entries {
            return false;
        }

        self.emitted += 1;
        true
    }

    pub fn is_done(&self) -> bool {
        self.emitted >= self.config.max_entries
    }

    pub fn emitted(&self) -> usize {
        self.emitted
    }
}
