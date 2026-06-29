use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodexState {
    Idle,
    Working,
    WaitingForConfirmation,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness {
    Fresh,
    Stale,
    Disconnected,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawStatusEvent {
    pub version: String,
    pub source: String,
    pub session_id: String,
    pub sequence: u64,
    pub observed_at: DateTime<Utc>,
    pub state: CodexState,
    pub freshness: Freshness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodexStatusSignal {
    pub version: String,
    pub source: String,
    pub session_id: String,
    pub sequence: u64,
    pub observed_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub state: CodexState,
    pub freshness: Freshness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<HashMap<String, String>>,
}

impl RawStatusEvent {
    pub fn into_signal(self, received_at: DateTime<Utc>) -> Result<CodexStatusSignal, String> {
        let signal = CodexStatusSignal {
            version: self.version,
            source: self.source,
            session_id: self.session_id,
            sequence: self.sequence,
            observed_at: self.observed_at,
            received_at,
            state: self.state,
            freshness: self.freshness,
            reason_code: self.reason_code,
            provider_metadata: self.provider_metadata,
        };
        signal.validate()?;
        Ok(signal)
    }
}

impl CodexStatusSignal {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != "1" {
            return Err("version must be 1".to_string());
        }
        if self.session_id.is_empty()
            || self.session_id.contains('/')
            || self.session_id.contains('\\')
        {
            return Err("session_id must be non-empty and path-safe".to_string());
        }
        let source_re = Regex::new(r"^[a-z0-9][a-z0-9._-]*$").expect("valid source regex");
        if !source_re.is_match(&self.source) {
            return Err("source must use lowercase provider identifier syntax".to_string());
        }
        if self.observed_at - self.received_at > Duration::seconds(5) {
            return Err("observed_at is too far in the future".to_string());
        }
        if self.state == CodexState::WaitingForConfirmation {
            self.ensure_sanitized()?;
        }
        self.ensure_sanitized()
    }

    pub fn is_fresh(&self) -> bool {
        self.freshness == Freshness::Fresh
    }

    fn ensure_sanitized(&self) -> Result<(), String> {
        for value in self.provider_metadata.iter().flat_map(|m| m.values()) {
            if contains_sensitive_material(value) {
                return Err("provider_metadata contains sensitive material".to_string());
            }
        }
        if let Some(reason) = &self.reason_code {
            if contains_sensitive_material(reason) {
                return Err("reason_code contains sensitive material".to_string());
            }
        }
        Ok(())
    }
}

pub fn contains_sensitive_material(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    ["secret", "token", "command:", "code:", "file:", "prompt:"]
        .iter()
        .any(|needle| lowered.contains(needle))
}
