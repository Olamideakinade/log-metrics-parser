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
            "FATAL" => 5,
            _ => 2,
        }
    }

    pub fn ingest(&mut self, entry: &LogEntry) {
        let entry_priority = Self::level_to_priority(&entry.level);
        if entry_priority < self.min_level_priority {
            return;
        }

        self.total_processed += 1;

        match entry.level.to_uppercase().as_str() {
            "ERROR" | "FATAL" => self.errors += 1,
            "WARN" => self.warnings += 1,
            _
			=> {}
        }

        if let Some(latency) = entry.latency_ms {
            self.latencies.push(latency);
        }
    }

    pub fn finalize(mut self) -> SummaryReport {
        self.latencies.sort_unstable();

        let p50 = Self::percentile(&self.latencies, 50.0);
        let p90 = Self::percentile(&self.latencies, 90.0);
        let p99 = Self::percentile(&self.latencies, 99.0);

        SummaryReport {
            total_processed: self.total_processed,
            errors: self.errors,
            warnings: self.warnings,
            p50_latency: p50,
            p90_latency: p90,
            p99_latency: p99,
        }
    }

    fn percentile(sorted_data: &[u64], percentile: f64) -> u64 {
        if sorted_data.is_empty() {
            return 0;
        }
        let index = ((percentile / 100.0) * (sorted_data.len() as f64)).ceil() as usize;
        let index = index.saturating_sub(1);
        sorted_data[index.min(sorted_data.len() - 1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregator_metrics() {
        let mut agg = MetricAggregator::new("INFO");
        
        agg.ingest(&LogEntry {
            timestamp: "2025-02-17T00:00:00Z".to_string(),
            level: "INFO".to_string(),
            message: "test info".to_string(),
            latency_ms: Some(10),
            path: None,
        });

        agg.ingest(&LogEntry {
            timestamp: "2025-02-17T00:00:01Z".to_string(),
            level: "ERROR".to_string(),
            message: "test error".to_string(),
            latency_ms: Some(100),
            path: None,
        });

        let report = agg.finalize();
        assert_eq!(report.total_processed, 2);
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.p50_latency, 10);
        assert_eq!(report.p99_latency, 100);
    }
}
