/// Configuration for sliding or tumbling log windows.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowConfig {
    /// Number of entries per window.
    pub size: usize,
    /// Number of entries to advance the window (for sliding windows).
    pub step: usize,
    /// Whether to emit partial windows on flush.
    pub emit_partial: bool,
}

impl WindowConfig {
    pub fn tumbling(size: usize) -> Self {
        Self {
            size,
            step: size,
            emit_partial: false,
        }
    }

    pub fn sliding(size: usize, step: usize) -> Self {
        assert!(step > 0, "step must be greater than zero");
        assert!(step <= size, "step must not exceed window size");
        Self {
            size,
            step,
            emit_partial: false,
        }
    }

    pub fn with_emit_partial(mut self, emit_partial: bool) -> Self {
        self.emit_partial = emit_partial;
        self
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::tumbling(10)
    }
}
