# Changelog

All notable changes to this project will be documented in this file.

## [1.3.0] - 2025-02-23

### Added
- Real-time progress animations for log processing using `indicatif`.
- ANSI colored output for CLI metrics via `console`.
- Interactive visual summary report with improved legibility.

### Changed
- Refactored CLI loop to utilize iterator patterns for better performance.

## [1.2.0] - 2025-02-21
- Prometheus text exposition format export via `--prometheus` flag.
- Time-window filtering using `--start-time` and `--end-time` options.