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
            min_level_priority: Self::level_to_priority(min_level),
            total_processed: 0,
            errors: 0,
            warnings: 0,
            latencies: Vec::new(),
        }
    }

    fn level_to_priority(level: &str) -> u8 {
        match level.to_uppercase().as_str() {
            "DEBUG" => 1,
            "INFO" => 2,
            "WARN" => 3,
            "ERROR" => 4,
            "FATAL" => 5,
            _ => 2,
        }
    }

    pub fn process_entry(&mut self, entry: &LogEntry) {
        let priority = Self::level_to_priority(&entry.level);
        if priority < self.min_level_priority {
            return;
        }

        self.total_processed += 1;

        if priority >= 4 {
            self.errors += 1;
        } else if priority == 3 {
            self.warnings += 1;
        }

        if let Some(lat) = entry.latency_ms {
            self.latencies.push(lat);
        }
    }

    fn calculate_percentile(&self, percentile: f64) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let idx = ((percentile / 100.0) * (sorted.len() as f64)).ceil() as usize;
        let clamped_idx = idx.saturating_sub(1).min(sorted.len() - 1);
        sorted[clamped_idx]
    }

    pub fn generate_report(&self) -> SummaryReport {
        let p50 = self.calculate_percentile(50.0);
        let p90 = self.calculate_percentile(90.0);
        let p99 = self.calculate_percentile(99.0);

        let mean_latency = if self.latencies.is_empty() {
            0.0
        } else {
            self.latencies.iter().sum::<u64>() as f64 / self.latencies.len() as f64
        };

        let max_latency = self.latencies.iter().cloned().max().unwrap_or(0);

        SummaryReport {
            total_processed: self.total_processed,
            errors: self.errors,
            warnings: self.warnings,
            p50_latency: p50,
            p90_latency: p90,
            p99_latency: p99,
            mean_latency,
            max_latency,
        }
    }

    pub fn generate_prometheus_output(&self) -> String {
        let report = self.generate_report();
        format!(
            "# HELP log_metrics_total_processed Total logs processed\n\
             # TYPE log_metrics_total_processed counter\n\
             log_metrics_total_processed {}\n\
             # HELP log_metrics_errors Total error logs\n\
             # TYPE log_metrics_errors counter\n\
             log_metrics_errors {}\n\
             # HELP log_metrics_warnings Total warning logs\n\
             # TYPE log_metrics_warnings counter\n\
             log_metrics_warnings {}\n\
             # HELP log_metrics_latency_ms Latency percentiles and stats\n\
             # TYPE log_metrics_latency_ms gauge\n\
             log_metrics_latency_ms{{quantile=\"p50\"}} {}\n\
             log_metrics_latency_ms{{quantile=\"p90\"}} {}\n\
             log_metrics_latency_ms{{quantile=\"p99\"}} {}\n\
             log_metrics_latency_ms{{quantile=\"mean\"}} {:.2}\n\
             log_metrics_latency_ms{{quantile=\"max\"}} {}\n",
            report.total_processed,
            report.errors,
            report.warnings,
            report.p50_latency,
            report.p90_latency,
            report.p99_latency,
            report.mean_latency,
            report.max_latency
        )
    }
}
