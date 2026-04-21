#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::annotate::Annotator;
    use crate::annotate_config::AnnotateConfig;
    use crate::log_entry::LogEntry;

    fn make_entry(fields: &[(&str, &str)]) -> LogEntry {
        let mut map = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        LogEntry { fields: map, raw: String::new(), timestamp: None }
    }

    #[test]
    fn test_annotate_static_default() {
        let config = AnnotateConfig::new("env").with_default("production");
        let annotator = Annotator::new(config);
        let entry = make_entry(&[("level", "info")]);
        let result = annotator.annotate(entry);
        assert_eq!(result.fields.get("env").map(String::as_str), Some("production"));
    }

    #[test]
    fn test_annotate_from_source_field() {
        let config = AnnotateConfig::new("service_tag").with_source("service");
        let annotator = Annotator::new(config);
        let entry = make_entry(&[("service", "auth")]);
        let result = annotator.annotate(entry);
        assert_eq!(result.fields.get("service_tag").map(String::as_str), Some("auth"));
    }

    #[test]
    fn test_annotate_with_prefix_and_suffix() {
        let config = AnnotateConfig::new("tagged")
            .with_source("host")
            .with_prefix("[")
            .with_suffix("]");
        let annotator = Annotator::new(config);
        let entry = make_entry(&[("host", "web-01")]);
        let result = annotator.annotate(entry);
        assert_eq!(result.fields.get("tagged").map(String::as_str), Some("[web-01]"));
    }

    #[test]
    fn test_annotate_missing_source_falls_back_to_default() {
        let config = AnnotateConfig::new("region")
            .with_source("dc")
            .with_default("unknown");
        let annotator = Annotator::new(config);
        let entry = make_entry(&[("level", "warn")]);
        let result = annotator.annotate(entry);
        assert_eq!(result.fields.get("region").map(String::as_str), Some("unknown"));
    }

    #[test]
    fn test_annotate_all_applies_to_each_entry() {
        let config = AnnotateConfig::new("version").with_default("v2");
        let annotator = Annotator::new(config);
        let entries = vec![
            make_entry(&[("msg", "start")]),
            make_entry(&[("msg", "stop")]),
        ];
        let results = annotator.annotate_all(entries);
        assert_eq!(results.len(), 2);
        for r in &results {
            assert_eq!(r.fields.get("version").map(String::as_str), Some("v2"));
        }
    }

    #[test]
    fn test_annotate_overwrites_existing_field() {
        let config = AnnotateConfig::new("level").with_default("debug");
        let annotator = Annotator::new(config);
        let entry = make_entry(&[("level", "error")]);
        let result = annotator.annotate(entry);
        assert_eq!(result.fields.get("level").map(String::as_str), Some("debug"));
    }
}
