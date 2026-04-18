mod filter;

use anyhow::Result;
use clap::Parser;
use filter::{FilterConfig, apply_filter};
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(name = "logslice", about = "Filter and slice structured log files")]
struct Cli {
    /// Input log file (reads stdin if omitted)
    #[arg(short, long)]
    file: Option<String>,

    /// Start time filter (RFC3339, e.g. 2024-01-01T00:00:00Z)
    #[arg(long)]
    from: Option<String>,

    /// End time filter (RFC3339, e.g. 2024-01-01T01:00:00Z)
    #[arg(long)]
    to: Option<String>,

    /// Field filter in key=value format (e.g. level=error)
    #[arg(short = 'f', long)]
    field: Option<String>,

    /// JSON timestamp field name
    #[arg(long, default_value = "timestamp")]
    time_field: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = FilterConfig::new(
        cli.from.as_deref(),
        cli.to.as_deref(),
        cli.field.as_deref(),
        &cli.time_field,
    )?;

    let reader: Box<dyn BufRead> = match &cli.file {
        Some(path) => Box::new(io::BufReader::new(File::open(path)?)),
        None => Box::new(io::BufReader::new(io::stdin())),
    };

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if apply_filter(&line, &config)? {
            println!("{}", line);
        }
    }

    Ok(())
}
