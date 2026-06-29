mod support;

use ai_traffic_lights::domain::{aggregate_from_signals, DisplayColor};
use ai_traffic_lights::providers::codex_app_server::map_codex_activity;
use ai_traffic_lights::providers::jsonl_replay::load_replay_file;
use support::replay::fixture_path;

#[test]
fn loads_basic_replay_sequence() {
    let events = load_replay_file(fixture_path("basic.jsonl")).expect("basic replay");
    assert_eq!(events.len(), 4);
    assert_eq!(
        events[2].state,
        ai_traffic_lights::domain::CodexState::WaitingForConfirmation
    );

    let display = aggregate_from_signals(&events);
    assert_eq!(display.display, DisplayColor::Green);
}

#[test]
fn loads_multi_session_priority_sequence_as_yellow() {
    let events = load_replay_file(fixture_path("multi-session-priority.jsonl"))
        .expect("multi-session replay");
    let display = aggregate_from_signals(&events);

    assert_eq!(display.display, DisplayColor::Yellow);
    assert_eq!(display.waiting_session_count, 1);
    assert_eq!(display.working_session_count, 1);
}

#[test]
fn loads_stale_source_sequence_as_unavailable() {
    let events = load_replay_file(fixture_path("stale-source.jsonl")).expect("stale replay");
    let display = aggregate_from_signals(&events);

    assert_eq!(display.display, DisplayColor::Unavailable);
    assert!(display.is_stale);
}

#[test]
fn rejects_non_monotonic_sequences() {
    let content = r#"
{"version":"1","source":"jsonl-replay","session_id":"demo-1","sequence":2,"observed_at":"2026-06-29T09:00:00Z","state":"working","freshness":"fresh"}
{"version":"1","source":"jsonl-replay","session_id":"demo-1","sequence":1,"observed_at":"2026-06-29T09:00:01Z","state":"idle","freshness":"fresh"}
"#;
    assert!(ai_traffic_lights::providers::jsonl_replay::parse_replay_jsonl(content).is_err());
}

#[test]
fn maps_confirmed_app_server_methods_to_status_states() {
    assert_eq!(
        map_codex_activity("item/commandExecution/requestApproval"),
        ai_traffic_lights::domain::CodexState::WaitingForConfirmation
    );
    assert_eq!(
        map_codex_activity("item/fileChange/requestApproval"),
        ai_traffic_lights::domain::CodexState::WaitingForConfirmation
    );
    assert_eq!(
        map_codex_activity("item/tool/requestUserInput"),
        ai_traffic_lights::domain::CodexState::WaitingForConfirmation
    );
    assert_eq!(
        map_codex_activity("mcpServer/elicitation/request"),
        ai_traffic_lights::domain::CodexState::WaitingForConfirmation
    );
    assert_eq!(
        map_codex_activity("turn/start"),
        ai_traffic_lights::domain::CodexState::Working
    );
}
