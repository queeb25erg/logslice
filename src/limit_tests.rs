#[cfg(test)]
mod tests {
    use crate::limit::{LimitConfig, Limiter};

    #[test]
    fn test_accept_all_when_unlimited() {
        let mut limiter = Limiter::new(LimitConfig::unlimited());
        for _ in 0..1000 {
            assert!(limiter.accept());
        }
    }

    #[test]
    fn test_limits_entries() {
        let mut limiter = Limiter::new(LimitConfig::new(3, 0));
        assert!(limiter.accept());
        assert!(limiter.accept());
        assert!(limiter.accept());
        assert!(!limiter.accept());
        assert!(!limiter.accept());
        assert_eq!(limiter.emitted(), 3);
    }

    #[test]
    fn test_offset_skips_entries() {
        let mut limiter = Limiter::new(LimitConfig::new(2, 3));
        // entries 0,1,2 skipped
        assert!(!limiter.accept());
        assert!(!limiter.accept());
        assert!(!limiter.accept());
        // entries 3,4 accepted
        assert!(limiter.accept());
        assert!(limiter.accept());
        // entry 5 rejected (limit reached)
        assert!(!limiter.accept());
        assert_eq!(limiter.emitted(), 2);
    }

    #[test]
    fn test_is_done_when_limit_reached() {
        let mut limiter = Limiter::new(LimitConfig::new(1, 0));
        assert!(!limiter.is_done());
        limiter.accept();
        assert!(limiter.is_done());
    }

    #[test]
    fn test_zero_limit_accepts_nothing() {
        let mut limiter = Limiter::new(LimitConfig::new(0, 0));
        assert!(!limiter.accept());
        assert_eq!(limiter.emitted(), 0);
    }

    #[test]
    fn test_offset_larger_than_input() {
        let mut limiter = Limiter::new(LimitConfig::new(5, 100));
        for _ in 0..50 {
            assert!(!limiter.accept());
        }
        assert_eq!(limiter.emitted(), 0);
    }
}
