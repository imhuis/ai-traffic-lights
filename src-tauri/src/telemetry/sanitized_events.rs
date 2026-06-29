use crate::domain::display_state::TrafficLightDisplayState;
use crate::domain::status_signal::contains_sensitive_material;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SanitizedTraceEvent {
    pub event: String,
    pub provider: String,
    pub display: String,
    pub active_session_count: usize,
    pub waiting_session_count: usize,
    pub working_session_count: usize,
    pub error_code: Option<String>,
}

impl SanitizedTraceEvent {
    pub fn from_display_state(
        provider: impl Into<String>,
        state: &TrafficLightDisplayState,
    ) -> Result<Self, String> {
        let provider = provider.into();
        let error_code = state.unavailable_reason.clone();
        if contains_sensitive_material(&provider)
            || error_code
                .as_deref()
                .map(contains_sensitive_material)
                .unwrap_or(false)
        {
            return Err("trace event contains sensitive material".to_string());
        }
        Ok(Self {
            event: "display_state_changed".to_string(),
            provider,
            display: format!("{:?}", state.display).to_ascii_lowercase(),
            active_session_count: state.active_session_count,
            waiting_session_count: state.waiting_session_count,
            working_session_count: state.working_session_count,
            error_code,
        })
    }
}
