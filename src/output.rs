use crate::log_entry::LogEntry;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Raw,
    Pretty,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "raw" => Some(OutputFormat::Raw),
            "pretty" => Some(OutputFormat::Pretty),
            _ => None,
        }
    }
}

pub struct OutputWriter<W: Write> {
    writer: W,
    format: OutputFormat,
    count: usize,
}

impl<W: Write> OutputWriter<W> {
    pub fn new(writer: W, format: OutputFormat) -> Self {
        OutputWriter { writer, format, count: 0 }
    }

    pub fn write_entry(&mut self, entry: &LogEntry) -> io::Result<()> {
        self.count += 1;
        match self.format {
            OutputFormat::Json => {
                writeln!(self.writer, "{}", entry.raw.trim())
            }
            OutputFormat::Raw => {
                writeln!(self.writer, "{}", entry.raw.trim())
            }
            OutputFormat::Pretty => {
                let ts = entry.timestamp
                    .map(|t| t.to_rfc3339())
                    .unwrap_or_else(|| "<no timestamp>".to_string());
                writeln!(
                    self.writer,
                    "[{}] {}",
                    ts,
                    entry.raw.trim()
                )
            }
        }
    }

    pub fn write_all(&mut self, entries: &[LogEntry]) -> io::Result<()> {
        for entry in entries {
            self.write_entry(entry)?;
        }
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub fn default_writer() -> OutputWriter<io::Stdout> {
    OutputWriter::new(io::stdout(), OutputFormat::Raw)
}
