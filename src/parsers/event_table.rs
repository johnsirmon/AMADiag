use serde::Serialize;

/// An entry from the MAEventTable / MetricsExtension ETW CSV output.
#[derive(Debug, Clone, Serialize)]
pub struct EventEntry {
    pub timestamp: Option<String>,
    pub level: Option<u32>,
    pub source: Option<String>,
    pub message: String,
}

/// Parse CSV content (e.g., MetricsExtension ETW trace CSV) into event entries.
pub fn parse_csv(content: &str) -> anyhow::Result<Vec<EventEntry>> {
    let mut entries = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()?.clone();

    // Find column indices by name (case-insensitive)
    let ts_idx = find_col(&headers, &["timestamp", "time", "datetime"]);
    let level_idx = find_col(&headers, &["level", "severity", "eventlevel"]);
    let source_idx = find_col(&headers, &["source", "provider", "taskname"]);
    let msg_idx = find_col(&headers, &["message", "msg", "formattedmessage", "eventmessage"]);

    for record in reader.records().flatten() {
        let timestamp = ts_idx.and_then(|i| record.get(i).map(|s| s.to_string()));
        let level = level_idx
            .and_then(|i| record.get(i))
            .and_then(|s| s.parse::<u32>().ok());
        let source = source_idx.and_then(|i| record.get(i).map(|s| s.to_string()));
        let message = msg_idx
            .and_then(|i| record.get(i))
            .unwrap_or("")
            .to_string();

        if !message.is_empty() {
            entries.push(EventEntry {
                timestamp,
                level,
                source,
                message,
            });
        }
    }

    Ok(entries)
}

fn find_col(headers: &csv::StringRecord, candidates: &[&str]) -> Option<usize> {
    headers.iter().position(|h| {
        let lower = h.to_lowercase();
        candidates.iter().any(|c| lower.contains(c))
    })
}
