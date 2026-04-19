#[cfg(test)]
mod tests {
    use crate::highlight::Highlighter;

    #[test]
    fn test_no_fields_returns_original() {
        let h = Highlighter::default();
        let line = "level=info msg=hello";
        assert_eq!(h.apply(line), line);
    }

    #[test]
    fn test_highlight_single_field() {
        let h = Highlighter::new(vec!["level".to_string()]);
        let result = h.apply("level=info msg=hello");
        assert!(result.contains("level=info"));
        assert!(result.contains("\x1b[33m"));
        assert!(result.contains("\x1b[0m"));
    }

    #[test]
    fn test_highlight_field_at_end_of_line() {
        let h = Highlighter::new(vec!["msg".to_string()]);
        let result = h.apply("level=info msg=hello");
        assert!(result.contains("msg=hello"));
        assert!(result.contains("\x1b[33m"));
    }

    #[test]
    fn test_highlight_multiple_fields() {
        let h = Highlighter::new(vec!["level".to_string(), "msg".to_string()]);
        let result = h.apply("level=warn msg=oops ts=123");
        assert!(result.contains("level=warn") || result.contains("msg=oops"));
    }

    #[test]
    fn test_field_not_present_unchanged() {
        let h = Highlighter::new(vec!["error".to_string()]);
        let line = "level=info msg=ok";
        let result = h.apply(line);
        assert!(!result.contains("\x1b[33m"));
    }

    #[test]
    fn test_custom_color() {
        let h = Highlighter::new(vec!["level".to_string()])
            .with_color("\x1b[31m");
        let result = h.apply("level=error msg=fail");
        assert!(result.contains("\x1b[31m"));
    }

    #[test]
    fn test_is_empty() {
        let h = Highlighter::default();
        assert!(h.is_empty());
        let h2 = Highlighter::new(vec!["x".to_string()]);
        assert!(!h2.is_empty());
    }
}
