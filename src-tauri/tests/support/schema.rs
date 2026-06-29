use ai_traffic_lights::domain::{
    CodexStatusSignal, DisplayColor, RawStatusEvent, TrafficLightDisplayState,
};
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn contract_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../specs/001-codex-status-light/contracts")
        .join(name)
}

pub fn load_contract(name: &str) -> Value {
    let text = std::fs::read_to_string(contract_path(name)).expect("contract file");
    serde_json::from_str(&text).expect("contract is json")
}

pub fn validate_status_event(value: &Value) -> Result<CodexStatusSignal, String> {
    let allowed: HashSet<&str> = [
        "version",
        "source",
        "session_id",
        "sequence",
        "observed_at",
        "state",
        "freshness",
        "reason_code",
        "provider_metadata",
    ]
    .into_iter()
    .collect();
    reject_unknown_keys(value, &allowed)?;
    let raw: RawStatusEvent =
        serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    let received_at = raw.observed_at;
    raw.into_signal(received_at)
}

pub fn validate_display_state(value: &Value) -> Result<TrafficLightDisplayState, String> {
    let allowed: HashSet<&str> = [
        "version",
        "display",
        "label",
        "accessibility_label",
        "mapped_meaning",
        "last_update",
        "is_stale",
        "active_session_count",
        "waiting_session_count",
        "working_session_count",
        "unavailable_reason",
    ]
    .into_iter()
    .collect();
    reject_unknown_keys(value, &allowed)?;
    let state: TrafficLightDisplayState =
        serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    if state.version != "1" {
        return Err("version must be 1".to_string());
    }
    if state.label.is_empty()
        || state.accessibility_label.is_empty()
        || state.mapped_meaning.is_empty()
    {
        return Err("display labels must be present".to_string());
    }
    match state.display {
        DisplayColor::Yellow => {
            if state.last_update.is_none() || state.waiting_session_count == 0 || state.is_stale {
                return Err("yellow display requires fresh waiting session".to_string());
            }
        }
        DisplayColor::Red => {
            if state.last_update.is_none()
                || state.waiting_session_count != 0
                || state.working_session_count == 0
                || state.is_stale
            {
                return Err("red display requires fresh working session".to_string());
            }
        }
        DisplayColor::Green => {
            if state.last_update.is_none()
                || state.waiting_session_count != 0
                || state.working_session_count != 0
                || state.active_session_count == 0
                || state.is_stale
            {
                return Err("green display requires fresh idle sessions".to_string());
            }
        }
        DisplayColor::Unavailable => {}
    }
    Ok(state)
}

pub fn validate_validation_sequence(value: &Value) -> Result<(), String> {
    let events = value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| "events array is required".to_string())?;
    let expected = value
        .get("expected_display_states")
        .and_then(Value::as_array)
        .ok_or_else(|| "expected_display_states array is required".to_string())?;
    if events.is_empty() || expected.is_empty() {
        return Err("validation sequence arrays cannot be empty".to_string());
    }
    for event in events {
        validate_status_event(event)?;
    }
    for display in expected {
        validate_display_state(display)?;
    }
    Ok(())
}

fn reject_unknown_keys(value: &Value, allowed: &HashSet<&str>) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "expected object".to_string())?;
    for key in object.keys() {
        if !allowed.contains(key.as_str()) {
            return Err(format!("unknown key {key}"));
        }
    }
    Ok(())
}
