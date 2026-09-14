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
            _ => 2,
        }
    }

    pub fn ingest(&mut self, entry: LogEntry) {
        let entry_priority = Self::level_to_priority(&entry.level);
        if entry_priority < self.min_level_priority {
            return;
        }

        self.total_processed += 1;

        match entry.level.to_uppercase().as_str() {
            "ERROR" => self.errors += 1,
            "WARN" => self.warnings += 1,
            _ => {}
        }

        if let Some(lat) = entry.latency_ms {
            self.latencies.push(lat);
        }
    }

    fn calculate_percentile(&mut self, percentile: f64) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        self.latencies.sort_unstable();
        let index = (percentile * (self.latencies.len() - 1) as f64).round() as usize;
        self.latencies[index]
    }

    pub fn generate_summary(&mut self) -> SummaryReport {
        let p50 = self.calculate_percentile(0.50);
        let p90 = self.calculate_percentile(0.90);
        let p99 = self.calculate_percentile(0.99);

        SummaryReport {
            total_processed: self.total_processed,
            errors: self.errors,
            warnings: self.warnings,
            p50_latency: p50,
            p90_latency: p90,
            p99_latency: p99,
        }
    }

    pub fn print_text(&mut self) {
        let summary = self.generate_summary();
        println!("Log Metrics Summary");
        println!("-------------------");
        println!("Total Records Processed: {}", summary.total_processed);
        println!("Error Count:             {}", summary.errors);
        println!("Warning Count:           {}", summary.warnings);
        println!();
        println!("Latency Percentiles (ms):");
        println!("  P50: {}", summary.p50_latency);
        println!("  P90: {}", summary.p90_latency);
        println!("  P99: {}", summary.p99_latency);
    }

    pub fn print_json(&mut self) {
        let summary = self.generate_summary();
        let json_string = serde_json::to_string_pretty(&summary).unwrap();
        println!("{}", json_string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregator_filtering_and_counts() {
        let mut agg = MetricAggregator::new("WARN");

        let info_entry = LogEntry {
            timestamp: "2023-10-01T12:00:00Z".to_string(),
            level: "INFO".to_string(),
            message: "Ignore".to_string(),
            latency_ms: Some(10),
            path: None,
        };

        let warn_entry = LogEntry {
            timestamp: "2023-10-01T12:01:00Z".to_string(),
            level: "WARN".to_string(),
            message: "Watch out".to_string(),
            latency_ms: Some(50),
            path: None,
        };

        let error_entry = LogEntry {
            timestamp: "2023-10-01T12:02:00Z".to_string(),
            level: "ERROR".to_string(),
            message: "Fail".to_string(),
            latency_ms: Some(100),
            path: None,
        };

        agg.ingest(info_entry);
        agg.ingest(warn_entry);
        agg.ingest(error_entry);

        let summary = agg.generate_summary();
        assert_eq!(summary.total_processed, 2);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.errors, 1);
        assert_eq!(summary.p50_latency, 50);
        assert_eq!(summary.p99_latency, 100);
    }
}
