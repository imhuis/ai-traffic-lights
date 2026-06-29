mod support;

use ai_traffic_lights::domain::{aggregate_from_signals, DisplayColor};
use support::replay::signal_from_json;

#[test]
fn maps_idle_to_green_and_working_to_red() {
    let idle = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":1,"observed_at":"2026-06-29T09:00:00Z","state":"idle","freshness":"fresh"}"#,
    );
    let green = aggregate_from_signals(&[idle]);
    assert_eq!(green.display, DisplayColor::Green);
    assert_eq!(green.label, "Idle");

    let working = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":2,"observed_at":"2026-06-29T09:00:01Z","state":"working","freshness":"fresh"}"#,
    );
    let red = aggregate_from_signals(&[working]);
    assert_eq!(red.display, DisplayColor::Red);
    assert_eq!(red.working_session_count, 1);
}

#[test]
fn maps_missing_or_stale_sessions_to_unavailable() {
    let empty = aggregate_from_signals(&[]);
    assert_eq!(empty.display, DisplayColor::Unavailable);

    let stale = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":1,"observed_at":"2026-06-29T09:00:00Z","state":"unavailable","freshness":"stale","reason_code":"provider_stale"}"#,
    );
    let unavailable = aggregate_from_signals(&[stale]);
    assert_eq!(unavailable.display, DisplayColor::Unavailable);
    assert!(unavailable.is_stale);
}

#[test]
fn yellow_takes_priority_over_red_and_green() {
    let working = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":1,"observed_at":"2026-06-29T09:00:00Z","state":"working","freshness":"fresh"}"#,
    );
    let idle = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s2","sequence":1,"observed_at":"2026-06-29T09:00:01Z","state":"idle","freshness":"fresh"}"#,
    );
    let waiting = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s3","sequence":1,"observed_at":"2026-06-29T09:00:02Z","state":"waiting_for_confirmation","freshness":"fresh"}"#,
    );

    let display = aggregate_from_signals(&[working, idle, waiting]);
    assert_eq!(display.display, DisplayColor::Yellow);
    assert_eq!(display.waiting_session_count, 1);
    assert_eq!(display.working_session_count, 1);
}

#[test]
fn latest_stale_signal_supersedes_older_fresh_signal() {
    let working = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":1,"observed_at":"2026-06-29T09:00:00Z","state":"working","freshness":"fresh"}"#,
    );
    let stale = signal_from_json(
        r#"{"version":"1","source":"jsonl-replay","session_id":"s1","sequence":2,"observed_at":"2026-06-29T09:00:30Z","state":"unavailable","freshness":"disconnected","reason_code":"provider_disconnected"}"#,
    );

    let display = aggregate_from_signals(&[working, stale]);
    assert_eq!(display.display, DisplayColor::Unavailable);
}
