# logslice

Fast CLI tool for filtering and slicing structured log files by time range or field value.

---

## Installation

```bash
cargo install logslice
```

Or build from source:

```bash
git clone https://github.com/you/logslice && cd logslice && cargo build --release
```

## Usage

```bash
# Filter logs by time range
logslice --from "2024-01-15T08:00:00" --to "2024-01-15T09:00:00" app.log

# Filter by field value
logslice --field level=error app.log

# Combine filters and output to file
logslice --from "2024-01-15T08:00:00" --field service=api app.log -o out.log
```

### Options

| Flag | Description |
|------|-------------|
| `--from` | Start of time range (ISO 8601) |
| `--to` | End of time range (ISO 8601) |
| `--field` | Filter by field value (`key=value`) |
| `-o, --output` | Write output to file instead of stdout |
| `--format` | Log format: `json`, `logfmt` (default: auto-detect) |

## Supported Formats

- JSON (newline-delimited)
- logfmt

## License

MIT © 2024