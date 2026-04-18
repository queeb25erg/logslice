#[cfg(test)]
mod tests {
    use super::super::filter::{FilterConfig, apply_filter};

    fn make_log(ts: &str, level: &str) -> String {
        format!(r#"{{"timestamp":"{}","level":"{}","msg":"test"}}", ts, level)
    }

    #[test]
    fn test_no_filter_passes_all() {
        let config = FilterConfig::new(None, None, None, "timestamp").unwrap();
        let line = make_log("2024-01-01T00:00:00Z", "info");
        assert!(apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_from_filter_excludes_earlier() {
        let config = FilterConfig::new(Some("2024-01-01T01:00:00Z"), None, None, "timestamp").unwrap();
        let line = make_log("2024-01-01T00:00:00Z", "info");
        assert!(!apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_to_filter_excludes_later() {
        let config = FilterConfig::new(None, Some("2024-01-01T00:30:00Z"), None, "timestamp").unwrap();
        let line = make_log("2024-01-01T01:00:00Z", "info");
        assert!(!apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_time_range_includes_within() {
        let config = FilterConfig::new(
            Some("2024-01-01T00:00:00Z"),
            Some("2024-01-01T02:00:00Z"),
            None,
            "timestamp",
        ).unwrap();
        let line = make_log("2024-01-01T01:00:00Z", "info");
        assert!(apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_field_filter_match() {
        let config = FilterConfig::new(None, None, Some("level=error"), "timestamp").unwrap();
        let line = make_log("2024-01-01T00:00:00Z", "error");
        assert!(apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_field_filter_no_match() {
        let config = FilterConfig::new(None, None, Some("level=error"), "timestamp").unwrap();
        let line = make_log("2024-01-01T00:00:00Z", "info");
        assert!(!apply_filter(&line, &config).unwrap());
    }

    #[test]
    fn test_invalid_json_excluded() {
        let config = FilterConfig::new(None, None, None, "timestamp").unwrap();
        assert!(!apply_filter("not json at all", &config).unwrap());
    }

    #[test]
    fn test_invalid_field_format() {
        let result = FilterConfig::new(None, None, Some("badformat"), "timestamp");
        assert!(result.is_err());
    }
}
