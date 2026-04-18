use crate::format::OutputFormat;

/// Determines the output format from CLI args or file extension hints.
pub struct FormatSelector;

impl FormatSelector {
    /// Select format from explicit flag, falling back to default.
    pub fn select(flag: Option<&str>, filename_hint: Option<&str>) -> OutputFormat {
        if let Some(f) = flag {
            if let Some(fmt) = OutputFormat::from_str(f) {
                return fmt;
            }
        }
        if let Some(name) = filename_hint {
            if let Some(fmt) = Self::from_extension(name) {
                return fmt;
            }
        }
        OutputFormat::Plain
    }

    fn from_extension(filename: &str) -> Option<OutputFormat> {
        let ext = filename.rsplit('.').next()?;
        match ext.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "csv" => Some(OutputFormat::Csv),
            "txt" | "log" => Some(OutputFormat::Plain),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::OutputFormat;

    #[test]
    fn test_select_explicit_json() {
        assert_eq!(FormatSelector::select(Some("json"), None), OutputFormat::Json);
    }

    #[test]
    fn test_select_explicit_csv() {
        assert_eq!(FormatSelector::select(Some("csv"), None), OutputFormat::Csv);
    }

    #[test]
    fn test_select_from_extension_json() {
        assert_eq!(FormatSelector::select(None, Some("output.json")), OutputFormat::Json);
    }

    #[test]
    fn test_select_from_extension_csv() {
        assert_eq!(FormatSelector::select(None, Some("data.csv")), OutputFormat::Csv);
    }

    #[test]
    fn test_select_default_plain() {
        assert_eq!(FormatSelector::select(None, None), OutputFormat::Plain);
    }

    #[test]
    fn test_select_flag_overrides_extension() {
        assert_eq!(FormatSelector::select(Some("json"), Some("file.csv")), OutputFormat::Json);
    }

    #[test]
    fn test_select_unknown_extension_defaults_plain() {
        assert_eq!(FormatSelector::select(None, Some("file.xml")), OutputFormat::Plain);
    }
}
