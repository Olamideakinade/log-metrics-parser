use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    #[serde(default)]
    pub latency_ms: Option<u64>,
    #[serde(default)]
    pub path: Option<String>,
}

impl LogEntry {
    pub fn parse(raw_line: &str) -> Result<Self, String> {
        let entry: LogEntry = serde_json::from_str(raw_line)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        Ok(entry)
    }

    pub fn matches_time_window(&self, start: &Option<String>, end: &Option<String>) -> bool {
        if let Some(s) = start {
            if &self.timestamp < s {
                return false;
            }
        }
        if let Some(e) = end {
            if &self.timestamp > e {
                return false;
            }
        }
        true
    }
}
