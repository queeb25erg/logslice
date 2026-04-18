use crate::format::{format_entry, csv_header, OutputFormat};
use crate::log_entry::LogEntry;
use std::io::{self, Write};

pub struct Writer<W: Write> {
    inner: W,
    format: OutputFormat,
    header_written: bool,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W, format: OutputFormat) -> Self {
        Writer {
            inner,
            format,
            header_written: false,
        }
    }

    pub fn write_entry(&mut self, entry: &LogEntry) -> io::Result<()> {
        if self.format == OutputFormat::Csv && !self.header_written {
            writeln!(self.inner, "{}", csv_header())?;
            self.header_written = true;
        }
        let line = format_entry(entry, &self.format);
        writeln!(self.inner, "{}", line)
    }

    pub fn write_entries(&mut self, entries: &[LogEntry]) -> io::Result<usize> {
        let mut count = 0;
        for entry in entries {
            self.write_entry(entry)?;
            count += 1;
        }
        Ok(count)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub fn write_to_stdout(entries: &[LogEntry], format: OutputFormat) -> io::Result<usize> {
    let stdout = io::stdout();
    let mut writer = Writer::new(stdout.lock(), format);
    let count = writer.write_entries(entries)?;
    writer.flush()?;
    Ok(count)
}
