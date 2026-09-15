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
            if self.timestamp < *s {
                return false;
            }
        }
        if let Some(e) = end {
            if self.timestamp > *e {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let line = r#"{"timestamp":"2025-02-21T10:00:00Z","level":"INFO","message":"test","latency_ms":45}"#;
        let entry = LogEntry::parse(line).unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.latency_ms, Some(45));
    }

    #[test]
    fn test_parse_invalid_log() {
        let line = "not-json";
        assert!(LogEntry::parse(line).is_err());
    }

    #[test]
    fn test_time_window_filtering() {
        let entry = LogEntry {
            timestamp: "2025-02-21T12:00:00Z".to_string(),
            level: "INFO".to_string(),
            message: "msg".to_string(),
            latency_ms: None,
            path: None,
        };
        assert!(entry.matches_time_window(&Some("2025-02-21T11:00:00Z".to_string()), &Some("2025-02-21T13:00:00Z".to_string())));
        assert!(!entry.matches_time_window(&Some("2025-02-21T13:00:00Z".to_string()), &None));
    }
}
