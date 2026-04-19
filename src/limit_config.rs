use crate::limit::LimitConfig;

#[derive(Debug, Clone, Default)]
pub struct LimitArgs {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl LimitArgs {
    pub fn new(limit: Option<usize>, offset: Option<usize>) -> Self {
        Self { limit, offset }
    }

    pub fn to_config(&self) -> LimitConfig {
        let max_entries = self.limit.unwrap_or(usize::MAX);
        let offset = self.offset.unwrap_or(0);
        LimitConfig::new(max_entries, offset)
    }

    pub fn is_active(&self) -> bool {
        self.limit.is_some() || self.offset.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults_to_unlimited() {
        let args = LimitArgs::default();
        let cfg = args.to_config();
        assert_eq!(cfg.max_entries, usize::MAX);
        assert_eq!(cfg.offset, 0);
    }

    #[test]
    fn test_with_limit_and_offset() {
        let args = LimitArgs::new(Some(10), Some(5));
        let cfg = args.to_config();
        assert_eq!(cfg.max_entries, 10);
        assert_eq!(cfg.offset, 5);
    }

    #[test]
    fn test_is_active() {
        assert!(!LimitArgs::default().is_active());
        assert!(LimitArgs::new(Some(1), None).is_active());
        assert!(LimitArgs::new(None, Some(1)).is_active());
    }
}
