# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-02-17

### Added
- Machine-parseable JSON summary export via `--json-output` flag.
- Full percentile calculation for p50, p90, and p99 latency.
- Comprehensive unit tests for parsing invalid and valid structured log entries.
- Input file redirection support alongside existing stdin streaming.

### Changed
- Upgraded Cargo package version to 1.1.0.
- Refactored MetricAggregator internals for memory-efficient latency sorting.
