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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_log() {
        let raw = r#"{"timestamp":"2025-02-17T10:00:00Z","level":"INFO","message":"Startup complete","latency_ms":45,"path":"/health"}"#;
        let entry = LogEntry::parse(raw).unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "Startup complete");
        assert_eq!(entry.latency_ms, Some(45));
        assert_eq!(entry.path, Some("/health".to_string()));
    }

    #[test]
    fn test_parse_invalid_log() {
        let raw = "not-json-content";
        let result = LogEntry::parse(raw);
        assert!(result.is_err());
    }
}
