use crate::parser::LogEntry;
use serde::Serialize;

#[derive(Serialize)]
pub struct SummaryReport {
    pub total_processed: usize,
    pub errors: usize,
    pub warnings: usize,
    pub p50_latency: u64,
    pub p90_latency: u64,
    pub p99_latency: u64,
    pub mean_latency: f64,
    pub max_latency: u64,
}

pub struct MetricAggregator {
    min_level_priority: u8,
    total_processed: usize,
    errors: usize,
    warnings: usize,
    latencies: Vec<u64>,
}

impl MetricAggregator {
    pub fn new(min_level: &str) -> Self {
        Self {
            min_level_priority: Self::level_to_prio(min_level),
            total_processed: 0,
            errors: 0,
            warnings: 0,
            latencies: Vec::new(),
        }
    }

    fn level_to_prio(level: &str) -> u8 {
        match level.to_uppercase().as_str() {
            "TRACE" => 1,
            "DEBUG" => 2,
            "INFO" => 3,
            "WARN" => 4,
            "ERROR" => 5,
            "FATAL" => 6,
            _ => 3,
        }
    }

    pub fn process_entry(&mut self, entry: &LogEntry) {
        let prio = Self::level_to_prio(&entry.level);
        if prio < self.min_level_priority {
            return;
        }

        self.total_processed += 1;
        if entry.level.eq_ignore_ascii_case("ERROR") || entry.level.eq_ignore_ascii_case("FATAL") {
            self.errors += 1;
        } else if entry.level.eq_ignore_ascii_case("WARN") {
            self.warnings += 1;
        }

        if let Some(lat) = entry.latency_ms {
            self.latencies.push(lat);
        }
    }

    pub fn finalize(&mut self) -> SummaryReport {
        self.latencies.sort_unstable();

        let total = self.latencies.len();
        let p50_latency = if total > 0 { self.latencies[total * 50 / 100] } else { 0 };
        let p90_latency = if total > 0 { self.latencies[total * 90 / 100] } else { 0 };
        let p99_latency = if total > 0 { self.latencies[total * 99 / 100] } else { 0 };
        let max_latency = if total > 0 { *self.latencies.last().unwrap() } else { 0 };
        
        let sum_latency: u64 = self.latencies.iter().sum();
        let mean_latency = if total > 0 { sum_latency as f64 / total as f64 } else { 0.0 };

        SummaryReport {
            total_processed: self.total_processed,
            errors: self.errors,
            warnings: self.warnings,
            p50_latency,
            p90_latency,
            p99_latency,
            mean_latency,
            max_latency,
        }
    }

    pub fn render_prometheus(&self, report: &SummaryReport) -> String {
        format!(
            "# HELP log_metrics_processed_total Total logs processed\n\
             # TYPE log_metrics_processed_total counter\n\
             log_metrics_processed_total {}\n\
             # HELP log_metrics_errors_total Total error logs\n\
             # TYPE log_metrics_errors_total counter\n\
             log_metrics_errors_total {}\n\
             # HELP log_metrics_warnings_total Total warning logs\n\
             # TYPE log_metrics_warnings_total counter\n\
             log_metrics_warnings_total {}\n\
             # HELP log_metrics_latency_p50 P50 latency in ms\n\
             # TYPE log_metrics_latency_p50 gauge\n\
             log_metrics_latency_p50 {}\n\
             # HELP log_metrics_latency_p99 P99 latency in ms\n\
             # TYPE log_metrics_latency_p99 gauge\n\
             log_metrics_latency_p99 {}\n",
            report.total_processed, report.errors, report.warnings, report.p50_latency, report.p99_latency
        )
    }
}
