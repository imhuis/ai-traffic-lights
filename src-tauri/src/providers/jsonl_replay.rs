use crate::domain::status_signal::{CodexStatusSignal, RawStatusEvent};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_replay_file(path: impl AsRef<Path>) -> Result<Vec<CodexStatusSignal>, String> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|error| format!("failed to read replay file: {error}"))?;
    parse_replay_jsonl(&content)
}

pub fn parse_replay_jsonl(content: &str) -> Result<Vec<CodexStatusSignal>, String> {
    let mut signals = Vec::new();
    let mut last_sequence: HashMap<(String, String), u64> = HashMap::new();
    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let raw: RawStatusEvent = serde_json::from_str(trimmed)
            .map_err(|error| format!("line {} is not a status event: {error}", idx + 1))?;
        let received_at = raw.observed_at;
        let key = (raw.source.clone(), raw.session_id.clone());
        if let Some(previous) = last_sequence.get(&key) {
            if raw.sequence <= *previous {
                return Err(format!("line {} has non-monotonic sequence", idx + 1));
            }
        }
        last_sequence.insert(key, raw.sequence);
        signals.push(raw.into_signal(received_at)?);
    }
    Ok(signals)
}
