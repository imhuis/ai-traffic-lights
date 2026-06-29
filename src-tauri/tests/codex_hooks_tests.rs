use ai_traffic_lights::domain::{aggregate_from_signals, DisplayColor};
use ai_traffic_lights::providers::codex_hooks::{
    hooks_display_state_at, inspect_installation_at, install_hooks_at, load_status_file,
    parse_hooks_status_jsonl,
};
use chrono::{Duration, Utc};
use tempfile::tempdir;

#[test]
fn installs_codex_hooks_without_removing_existing_hooks() {
    let dir = tempdir().expect("tempdir");
    let hooks_json = dir.path().join("hooks.json");
    std::fs::write(
        &hooks_json,
        r#"{
  "hooks": {
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "python3 existing.py",
            "timeout": 15
          }
        ]
      }
    ]
  }
}
"#,
    )
    .expect("seed hooks json");

    let status = install_hooks_at(dir.path()).expect("install hooks");
    assert!(status.installed);
    assert!(status.review_required);
    assert!(status.missing_events.is_empty());

    let updated = std::fs::read_to_string(hooks_json).expect("updated hooks json");
    assert!(updated.contains("existing.py"));
    assert!(updated.contains("ai-traffic-lights-status.py"));
    assert!(updated.contains("PermissionRequest"));
    assert!(updated.contains("Stop"));
}

#[test]
fn reports_missing_events_before_install() {
    let dir = tempdir().expect("tempdir");
    let status = inspect_installation_at(dir.path());

    assert!(!status.installed);
    assert!(status
        .missing_events
        .contains(&"UserPromptSubmit".to_string()));
}

#[test]
fn loads_codex_hooks_status_file_as_display_state() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("status.jsonl");
    let observed_at = Utc::now().to_rfc3339();
    std::fs::write(
        &path,
        format!(
            "{{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":1,\"observed_at\":\"{observed_at}\",\"state\":\"waiting_for_confirmation\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"PermissionRequest\"}}}}\n"
        ),
    )
    .expect("write status file");

    let signals = load_status_file(path).expect("load hooks status file");
    let display = aggregate_from_signals(&signals);

    assert_eq!(display.display, DisplayColor::Yellow);
    assert_eq!(display.waiting_session_count, 1);
}

#[test]
fn keeps_recent_working_event_visible_after_fast_stop() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("status.jsonl");
    let now = Utc::now();
    std::fs::write(
        &path,
        format!(
            "{{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":1,\"observed_at\":\"{}\",\"state\":\"working\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"UserPromptSubmit\"}}}}\n\
             {{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":2,\"observed_at\":\"{}\",\"state\":\"idle\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"Stop\"}}}}\n",
            (now - Duration::milliseconds(250)).to_rfc3339(),
            (now - Duration::milliseconds(100)).to_rfc3339(),
        ),
    )
    .expect("write status file");

    let signals = load_status_file(path).expect("load hooks status file");
    let display = hooks_display_state_at(&signals, now);

    assert_eq!(display.display, DisplayColor::Red);
    assert_eq!(display.working_session_count, 1);
}

#[test]
fn does_not_hold_old_working_event_after_stop() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("status.jsonl");
    let now = Utc::now();
    std::fs::write(
        &path,
        format!(
            "{{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":1,\"observed_at\":\"{}\",\"state\":\"working\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"UserPromptSubmit\"}}}}\n\
             {{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":2,\"observed_at\":\"{}\",\"state\":\"idle\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"Stop\"}}}}\n",
            (now - Duration::seconds(4)).to_rfc3339(),
            (now - Duration::seconds(3)).to_rfc3339(),
        ),
    )
    .expect("write status file");

    let signals = load_status_file(path).expect("load hooks status file");
    let display = hooks_display_state_at(&signals, now);

    assert_eq!(display.display, DisplayColor::Green);
}

#[test]
fn live_hooks_parser_tolerates_non_monotonic_history() {
    let observed_at = Utc::now().to_rfc3339();
    let content = format!(
        "{{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":2,\"observed_at\":\"{observed_at}\",\"state\":\"working\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"PreToolUse\"}}}}\n\
         {{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":1,\"observed_at\":\"{observed_at}\",\"state\":\"working\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"PreToolUse\"}}}}\n"
    );

    let signals = parse_hooks_status_jsonl(&content).expect("parse live hooks status");

    assert_eq!(signals.len(), 2);
}

#[test]
fn live_hooks_parser_skips_bad_lines_when_valid_events_exist() {
    let observed_at = Utc::now().to_rfc3339();
    let content = format!(
        "not-json\n\
         {{\"version\":\"1\",\"source\":\"codex-hooks\",\"session_id\":\"local-demo\",\"sequence\":1,\"observed_at\":\"{observed_at}\",\"state\":\"working\",\"freshness\":\"fresh\",\"provider_metadata\":{{\"hook_event\":\"PreToolUse\"}}}}\n"
    );

    let signals = parse_hooks_status_jsonl(&content).expect("parse live hooks status");

    assert_eq!(signals.len(), 1);
}
