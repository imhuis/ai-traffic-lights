mod support;

use serde_json::json;
use support::schema::{
    load_contract, validate_display_state, validate_status_event, validate_validation_sequence,
};

#[test]
fn contract_files_are_valid_json() {
    load_contract("status-event.v1.schema.json");
    load_contract("display-state.v1.schema.json");
    load_contract("validation-sequence.v1.schema.json");
}

#[test]
fn status_event_accepts_valid_replay_event() {
    let value = json!({
        "version": "1",
        "source": "jsonl-replay",
        "session_id": "demo-1",
        "sequence": 1,
        "observed_at": "2026-06-29T09:00:00Z",
        "state": "working",
        "freshness": "fresh"
    });

    assert!(validate_status_event(&value).is_ok());
}

#[test]
fn status_event_rejects_missing_session_id_and_sensitive_metadata() {
    let missing_session = json!({
        "version": "1",
        "source": "jsonl-replay",
        "sequence": 1,
        "observed_at": "2026-06-29T09:00:00Z",
        "state": "working",
        "freshness": "fresh"
    });
    assert!(validate_status_event(&missing_session).is_err());

    let sensitive = json!({
        "version": "1",
        "source": "jsonl-replay",
        "session_id": "demo-1",
        "sequence": 1,
        "observed_at": "2026-06-29T09:00:00Z",
        "state": "waiting_for_confirmation",
        "freshness": "fresh",
        "provider_metadata": {
            "prompt": "command: rm -rf secret"
        }
    });
    assert!(validate_status_event(&sensitive).is_err());
}

#[test]
fn display_state_rejects_stale_normal_colors() {
    let stale_red = json!({
        "version": "1",
        "display": "red",
        "label": "Working",
        "accessibility_label": "Codex is working",
        "mapped_meaning": "At least one session is working.",
        "last_update": "2026-06-29T09:00:00Z",
        "is_stale": true,
        "active_session_count": 1,
        "waiting_session_count": 0,
        "working_session_count": 1
    });

    assert!(validate_display_state(&stale_red).is_err());
}

#[test]
fn validation_sequence_validates_nested_contracts() {
    let sequence = json!({
        "version": "1",
        "name": "basic",
        "events": [{
            "version": "1",
            "source": "jsonl-replay",
            "session_id": "demo-1",
            "sequence": 1,
            "observed_at": "2026-06-29T09:00:00Z",
            "state": "idle",
            "freshness": "fresh"
        }],
        "expected_display_states": [{
            "version": "1",
            "display": "green",
            "label": "Idle",
            "accessibility_label": "Codex is idle",
            "mapped_meaning": "All trusted Codex sessions are idle.",
            "last_update": "2026-06-29T09:00:00Z",
            "is_stale": false,
            "active_session_count": 1,
            "waiting_session_count": 0,
            "working_session_count": 0
        }]
    });

    assert!(validate_validation_sequence(&sequence).is_ok());
}
