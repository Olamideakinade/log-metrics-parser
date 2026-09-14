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
    fn test_parse_valid_log_line() {
        let line = r#"{"timestamp":"2023-10-01T12:00:00Z","level":"INFO","message":"Request handled","latency_ms":45,"path":"/api/v1/users"}"#;
        let entry = LogEntry::parse(line).unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.latency_ms, Some(45));
        assert_eq!(entry.path, Some("/api/v1/users".to_string()));
    }

    #[test]
    fn test_parse_invalid_json() {
        let line = "not-a-json-string";
        let result = LogEntry::parse(line);
        assert!(result.is_err());
    }
}
