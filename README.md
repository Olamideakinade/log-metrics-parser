# Log Metrics Parser

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/log-metrics-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`log-metrics-parser` is a command-line utility written in Rust designed to ingest JSON-structured log streams, filter entries by severity or latency thresholds, and output aggregate performance summaries.

## Key Capabilities

- **Stream Processing**: Efficiently reads standard input or file paths line-by-line with minimal memory allocation.
- **Aggregation**: Computes request counts, error rates, and latency percentiles (p50, p90, p99).
- **Configurable Filters**: Exclude or include log levels and specific routing paths via CLI flags.
- **Structured Output**: Prints clean text reports or exports results as formatted JSON for downstream CI/CD reporting.

## Output Demonstration

```bash
$ cat app.log | log-metrics-parser --min-level WARN --percentiles

Log Metrics Summary
-------------------
Total Records Processed: 12,450
Filtered Out:             8,100
Error Count:                142
Warning Count:              350

Latency Percentiles (ms):
  P50: 12.4
  P90: 45.1
  P99: 128.9
```

## Quickstart

Build and install locally using Cargo:

```bash
git clone https://github.com/Olamideakinade/log-metrics-parser.git
cd log-metrics-parser
cargo build --release

# Run against a sample log file
./target/release/log-metrics-parser --input sample.log --json
```

## Architecture & Design

The codebase is organized into modular components:
- `src/main.rs`: CLI argument parsing via `clap` and orchestration.
- `src/parser.rs`: Line-by-line deserialization and struct mapping.
- `src/aggregator.rs`: Statistical calculations and percentile distribution models.

## License

MIT