# Log Metrics Parser

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/log-metrics-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`log-metrics-parser` is a command-line utility written in Rust designed to ingest JSON-structured log streams, filter entries by severity or latency thresholds, and output aggregate performance summaries.

## Key Capabilities

- **Stream Processing**: Efficiently parses high-volume newline-delimited JSON logs from stdin or files.
- **Performance Percentiles**: Calculates exact p50, p90, p99, mean, and max latency figures.
- **Prometheus Export**: Exposes computed summaries directly in Prometheus text exposition format.
- **Time-Window Filtering**: Filter log items dynamically based on strict ISO-8601 timestamp bounds.

## Installation & Usage

```bash
cargo build --release

# Run with file input and JSON summary
./target/release/log-metrics-parser --input logs.json --json-output

# Export directly to Prometheus metrics format
./target/release/log-metrics-parser --input logs.json --prometheus
```
