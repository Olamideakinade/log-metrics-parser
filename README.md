# Log Metrics Parser

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/log-metrics-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`log-metrics-parser` is a command-line utility written in Rust designed to ingest JSON-structured log streams, filter entries by severity or latency thresholds, and output aggregate performance summaries.

## Key Capabilities

- **Stream Processing**: Efficient line-by-line log ingestion from stdin or files.
- **Severity Filtering**: Filter log items dynamically by severity level (DEBUG, INFO, WARN, ERROR, FATAL).
- **Performance Percentiles**: Automatically compute p50, p90, and p99 latency metrics.
- **Structured Export**: Human-readable text or machine-parseable JSON summary reports.

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Parse from a file with minimum level WARN and JSON output
log-metrics-parser --input logs.json --min-level WARN --json-output

# Pipe from stdin
cat logs.json | log-metrics-parser --min-level INFO
```
