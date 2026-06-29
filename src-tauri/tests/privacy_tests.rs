use ai_traffic_lights::domain::{RawStatusEvent, TrafficLightDisplayState, UserDisplayPreference};
use ai_traffic_lights::telemetry::sanitized_events::SanitizedTraceEvent;
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn rejects_sensitive_status_payload_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("prompt".to_string(), "secret token in prompt".to_string());
    let raw = RawStatusEvent {
        version: "1".to_string(),
        source: "jsonl-replay".to_string(),
        session_id: "demo-1".to_string(),
        sequence: 1,
        observed_at: Utc::now(),
        state: ai_traffic_lights::domain::CodexState::WaitingForConfirmation,
        freshness: ai_traffic_lights::domain::Freshness::Fresh,
        reason_code: None,
        provider_metadata: Some(metadata),
    };

    assert!(raw.into_signal(Utc::now()).is_err());
}

#[test]
fn trace_events_are_sanitized() {
    let state = TrafficLightDisplayState::unavailable("provider_disconnected", None);
    let event =
        SanitizedTraceEvent::from_display_state("jsonl-replay", &state).expect("sanitized event");
    assert_eq!(event.error_code.as_deref(), Some("provider_disconnected"));

    let rejected = SanitizedTraceEvent::from_display_state("secret-token-provider", &state);
    assert!(rejected.is_err());
}

#[test]
fn preferences_do_not_serialize_status_history() {
    let preference = UserDisplayPreference::default();
    let value = serde_json::to_string(&preference).expect("serialize preferences");

    assert!(!value.contains("Codex"));
    assert!(!value.contains("session_id"));
    assert!(!value.contains("command"));
}
