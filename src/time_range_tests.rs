#[cfg(test)]
mod tests {
    use crate::time_range::{parse_timestamp, TimeRange};
    use chrono::{TimeZone, Utc};

    fn ts(s: &str) -> chrono::DateTime<Utc> {
        parse_timestamp(s).unwrap()
    }

    #[test]
    fn test_parse_rfc3339() {
        let dt = parse_timestamp("2024-01-15T10:30:00Z").unwrap();
        assert_eq!(dt.year(), 2024);
        use chrono::Datelike;
        assert_eq!(dt.month(), 1);
    }

    #[test]
    fn test_parse_naive_datetime() {
        let dt = parse_timestamp("2024-03-20 08:15:00").unwrap();
        use chrono::Timelike;
        assert_eq!(dt.hour(), 8);
    }

    #[test]
    fn test_parse_date_only() {
        let dt = parse_timestamp("2024-06-01").unwrap();
        use chrono::Timelike;
        assert_eq!(dt.hour(), 0);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_timestamp("not-a-date").is_err());
    }

    #[test]
    fn test_range_contains() {
        let range = TimeRange::new(
            Some(ts("2024-01-01")),
            Some(ts("2024-01-31")),
        ).unwrap();
        assert!(range.contains(&ts("2024-01-15")));
        assert!(!range.contains(&ts("2024-02-01")));
        assert!(!range.contains(&ts("2023-12-31")));
    }

    #[test]
    fn test_range_unbounded_start() {
        let range = TimeRange::new(None, Some(ts("2024-06-01"))).unwrap();
        assert!(range.contains(&ts("2020-01-01")));
        assert!(!range.contains(&ts("2024-07-01")));
    }

    #[test]
    fn test_range_start_after_end_error() {
        let result = TimeRange::new(
            Some(ts("2024-06-01")),
            Some(ts("2024-01-01")),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_unbounded_range() {
        let range = TimeRange::new(None, None).unwrap();
        assert!(range.is_unbounded());
        assert!(range.contains(&ts("2024-01-01")));
    }
}
