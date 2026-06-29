use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayColor {
    Green,
    Red,
    Yellow,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficLightDisplayState {
    pub version: String,
    pub display: DisplayColor,
    pub label: String,
    pub accessibility_label: String,
    pub mapped_meaning: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime<Utc>>,
    pub is_stale: bool,
    pub active_session_count: usize,
    pub waiting_session_count: usize,
    pub working_session_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
}

impl TrafficLightDisplayState {
    pub fn green(last_update: DateTime<Utc>, active_session_count: usize) -> Self {
        Self {
            version: "1".to_string(),
            display: DisplayColor::Green,
            label: "Idle".to_string(),
            accessibility_label: "Codex is idle".to_string(),
            mapped_meaning: "All trusted Codex sessions are idle.".to_string(),
            last_update: Some(last_update),
            is_stale: false,
            active_session_count,
            waiting_session_count: 0,
            working_session_count: 0,
            unavailable_reason: None,
        }
    }

    pub fn red(last_update: DateTime<Utc>, active: usize, working: usize) -> Self {
        Self {
            version: "1".to_string(),
            display: DisplayColor::Red,
            label: "Working".to_string(),
            accessibility_label: "Codex is working".to_string(),
            mapped_meaning: "At least one trusted Codex session is working.".to_string(),
            last_update: Some(last_update),
            is_stale: false,
            active_session_count: active,
            waiting_session_count: 0,
            working_session_count: working,
            unavailable_reason: None,
        }
    }

    pub fn yellow(
        last_update: DateTime<Utc>,
        active: usize,
        waiting: usize,
        working: usize,
    ) -> Self {
        Self {
            version: "1".to_string(),
            display: DisplayColor::Yellow,
            label: "Needs confirmation".to_string(),
            accessibility_label: "Codex is waiting for confirmation".to_string(),
            mapped_meaning: "At least one Codex session needs user confirmation.".to_string(),
            last_update: Some(last_update),
            is_stale: false,
            active_session_count: active,
            waiting_session_count: waiting,
            working_session_count: working,
            unavailable_reason: None,
        }
    }

    pub fn unavailable(reason: impl Into<String>, last_update: Option<DateTime<Utc>>) -> Self {
        Self {
            version: "1".to_string(),
            display: DisplayColor::Unavailable,
            label: "Unavailable".to_string(),
            accessibility_label: "Codex status is unavailable".to_string(),
            mapped_meaning: "Current Codex status cannot be trusted.".to_string(),
            last_update,
            is_stale: true,
            active_session_count: 0,
            waiting_session_count: 0,
            working_session_count: 0,
            unavailable_reason: Some(reason.into()),
        }
    }
}
