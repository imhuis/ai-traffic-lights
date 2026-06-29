use crate::domain::aggregate_display_state;
use crate::domain::display_state::TrafficLightDisplayState;
use crate::domain::status_signal::{CodexState, CodexStatusSignal, RawStatusEvent};
use crate::providers::StatusProviderHealth;
use chrono::{DateTime, Duration, Utc};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const PROVIDER_NAME: &str = "codex-hooks";

const HOOK_SCRIPT_NAME: &str = "ai-traffic-lights-status.py";
const HOOK_SCRIPT_MARKER: &str = "ai-traffic-lights-status-hook-v1";
const HOOK_EVENTS: &[&str] = &[
    "UserPromptSubmit",
    "PreToolUse",
    "PermissionRequest",
    "PostToolUse",
    "Stop",
];
const TRANSIENT_ACTIVITY_HOLD: Duration = Duration::seconds(2);

#[derive(Debug, Clone)]
pub struct CodexHooksPaths {
    pub hooks_json_path: PathBuf,
    pub hook_script_path: PathBuf,
    pub status_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodexHooksInstallStatus {
    pub installed: bool,
    pub config_path: String,
    pub script_path: String,
    pub status_path: String,
    pub configured_events: Vec<String>,
    pub missing_events: Vec<String>,
    pub review_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

pub fn default_codex_home() -> Result<PathBuf, String> {
    if let Some(home) = std::env::var_os("CODEX_HOME") {
        return Ok(PathBuf::from(home));
    }

    BaseDirs::new()
        .map(|dirs| dirs.home_dir().join(".codex"))
        .ok_or_else(|| "codex_home_unavailable".to_string())
}

pub fn paths_for(codex_home: impl AsRef<Path>) -> CodexHooksPaths {
    let codex_home = codex_home.as_ref();
    CodexHooksPaths {
        hooks_json_path: codex_home.join("hooks.json"),
        hook_script_path: codex_home.join("hooks").join(HOOK_SCRIPT_NAME),
        status_path: codex_home.join("ai-traffic-lights").join("status.jsonl"),
    }
}

pub fn inspect_installation() -> CodexHooksInstallStatus {
    match default_codex_home() {
        Ok(home) => inspect_installation_at(home),
        Err(error) => missing_status(error),
    }
}

pub fn inspect_installation_at(codex_home: impl AsRef<Path>) -> CodexHooksInstallStatus {
    let paths = paths_for(codex_home);
    let script_installed = fs::read_to_string(&paths.hook_script_path)
        .map(|content| content.contains(HOOK_SCRIPT_MARKER))
        .unwrap_or(false);
    let configured_events = configured_hook_events(&paths.hooks_json_path, &paths.hook_script_path);
    let missing_events = HOOK_EVENTS
        .iter()
        .filter(|event| {
            !configured_events
                .iter()
                .any(|configured| configured == **event)
        })
        .map(|event| (*event).to_string())
        .collect::<Vec<_>>();
    let installed = script_installed && missing_events.is_empty();

    CodexHooksInstallStatus {
        installed,
        config_path: paths.hooks_json_path.display().to_string(),
        script_path: paths.hook_script_path.display().to_string(),
        status_path: paths.status_path.display().to_string(),
        configured_events,
        missing_events,
        review_required: installed,
        detail: if installed {
            Some("run_codex_hooks_review_if_not_trusted".to_string())
        } else if !script_installed {
            Some("hook_script_missing".to_string())
        } else {
            Some("hook_config_incomplete".to_string())
        },
    }
}

pub fn install_hooks() -> Result<CodexHooksInstallStatus, String> {
    let home = default_codex_home()?;
    install_hooks_at(home)
}

pub fn install_hooks_at(codex_home: impl AsRef<Path>) -> Result<CodexHooksInstallStatus, String> {
    let paths = paths_for(codex_home);
    if let Some(parent) = paths.hook_script_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create hooks dir: {error}"))?;
    }
    if let Some(parent) = paths.status_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create status dir: {error}"))?;
    }

    fs::write(&paths.hook_script_path, hook_script(&paths.status_path))
        .map_err(|error| format!("write hook script: {error}"))?;
    make_executable(&paths.hook_script_path)?;

    let mut config = read_hooks_json(&paths.hooks_json_path)?;
    ensure_hooks_config(&mut config, &paths.hook_script_path)?;
    let rendered = serde_json::to_string_pretty(&config)
        .map_err(|error| format!("render hooks json: {error}"))?;
    fs::write(&paths.hooks_json_path, format!("{rendered}\n"))
        .map_err(|error| format!("write hooks json: {error}"))?;

    Ok(inspect_installation_at(
        paths
            .hooks_json_path
            .parent()
            .ok_or_else(|| "codex_home_unavailable".to_string())?,
    ))
}

pub fn current_display_state() -> TrafficLightDisplayState {
    let install_status = inspect_installation();
    if !install_status.installed {
        return TrafficLightDisplayState::unavailable("codex_hooks_not_installed", None);
    }

    match load_status_file(Path::new(&install_status.status_path)) {
        Ok(signals) => hooks_display_state_at(&signals, Utc::now()),
        Err(error) => TrafficLightDisplayState::unavailable(error, None),
    }
}

pub fn load_status_file(
    path: impl AsRef<Path>,
) -> Result<Vec<crate::domain::CodexStatusSignal>, String> {
    if !path.as_ref().exists() {
        return Err("codex_hooks_no_status_file".to_string());
    }
    let content = fs::read_to_string(path.as_ref())
        .map_err(|error| format!("failed to read hooks status file: {error}"))?;
    parse_hooks_status_jsonl(&content)
}

pub fn parse_hooks_status_jsonl(content: &str) -> Result<Vec<CodexStatusSignal>, String> {
    let mut signals = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(raw) = serde_json::from_str::<RawStatusEvent>(trimmed) else {
            continue;
        };
        let received_at = raw.observed_at;
        if let Ok(signal) = raw.into_signal(received_at) {
            signals.push(signal);
        }
    }

    if signals.is_empty() {
        return Err("codex_hooks_no_valid_status_events".to_string());
    }
    Ok(signals)
}

pub fn hooks_display_state_at(
    signals: &[CodexStatusSignal],
    now: DateTime<Utc>,
) -> TrafficLightDisplayState {
    let display = aggregate_display_state(signals, &StatusProviderHealth::connected(PROVIDER_NAME));
    if display.display != crate::domain::display_state::DisplayColor::Green {
        return display;
    }

    let recent: Vec<&CodexStatusSignal> = signals
        .iter()
        .filter(|signal| {
            signal.is_fresh()
                && matches!(
                    signal.state,
                    CodexState::Working | CodexState::WaitingForConfirmation
                )
                && signal.observed_at <= now
                && now - signal.observed_at <= TRANSIENT_ACTIVITY_HOLD
        })
        .collect();

    if recent.is_empty() {
        return display;
    }

    let active = display.active_session_count.max(1);
    let last_update = recent
        .iter()
        .map(|signal| signal.observed_at)
        .max()
        .unwrap_or(now);
    let waiting = recent
        .iter()
        .filter(|signal| signal.state == CodexState::WaitingForConfirmation)
        .count();
    let working = recent
        .iter()
        .filter(|signal| signal.state == CodexState::Working)
        .count();

    if waiting > 0 {
        TrafficLightDisplayState::yellow(last_update, active, waiting, working)
    } else {
        TrafficLightDisplayState::red(last_update, active, working.max(1))
    }
}

fn missing_status(detail: String) -> CodexHooksInstallStatus {
    CodexHooksInstallStatus {
        installed: false,
        config_path: String::new(),
        script_path: String::new(),
        status_path: String::new(),
        configured_events: Vec::new(),
        missing_events: HOOK_EVENTS
            .iter()
            .map(|event| (*event).to_string())
            .collect(),
        review_required: false,
        detail: Some(detail),
    }
}

fn configured_hook_events(hooks_json_path: &Path, script_path: &Path) -> Vec<String> {
    let Ok(config) = fs::read_to_string(hooks_json_path) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<Value>(&config) else {
        return Vec::new();
    };
    let Some(hooks) = value.get("hooks").and_then(Value::as_object) else {
        return Vec::new();
    };

    HOOK_EVENTS
        .iter()
        .filter(|event| {
            hooks
                .get(**event)
                .and_then(Value::as_array)
                .map(|entries| event_has_hook(entries, script_path))
                .unwrap_or(false)
        })
        .map(|event| (*event).to_string())
        .collect()
}

fn event_has_hook(entries: &[Value], script_path: &Path) -> bool {
    entries.iter().any(|entry| {
        entry
            .get("hooks")
            .and_then(Value::as_array)
            .map(|hooks| {
                hooks.iter().any(|hook| {
                    hook.get("command")
                        .and_then(Value::as_str)
                        .map(|command| {
                            command.contains(HOOK_SCRIPT_NAME)
                                || command.contains(&script_path.display().to_string())
                        })
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    })
}

fn read_hooks_json(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(json!({ "hooks": {} }));
    }

    let content = fs::read_to_string(path).map_err(|error| format!("read hooks json: {error}"))?;
    serde_json::from_str(&content).map_err(|error| format!("hooks_json_invalid: {error}"))
}

fn ensure_hooks_config(config: &mut Value, script_path: &Path) -> Result<(), String> {
    if !config.is_object() {
        return Err("hooks_json_root_must_be_object".to_string());
    }
    if config.get("hooks").is_none() {
        config["hooks"] = json!({});
    }
    let hooks = config
        .get_mut("hooks")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "hooks_json_hooks_must_be_object".to_string())?;

    for event in HOOK_EVENTS {
        let entry = hooks
            .entry((*event).to_string())
            .or_insert_with(|| json!([]));
        let entries = entry
            .as_array_mut()
            .ok_or_else(|| format!("hooks_json_event_must_be_array:{event}"))?;
        if !event_has_hook(entries, script_path) {
            entries.push(json!({
                "hooks": [
                    {
                        "type": "command",
                        "command": hook_command(script_path, event),
                        "timeout": 5
                    }
                ]
            }));
        }
    }
    Ok(())
}

fn hook_command(script_path: &Path, event: &str) -> String {
    let interpreter = if cfg!(windows) { "py -3" } else { "python3" };
    format!(
        "{interpreter} {} {event}",
        shell_double_quote(&script_path.display().to_string())
    )
}

fn shell_double_quote(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\\\""))
}

fn hook_script(status_path: &Path) -> String {
    let status_path_literal =
        serde_json::to_string(&status_path.display().to_string()).expect("path serializes");
    format!(
        r#"#!/usr/bin/env python3
# {HOOK_SCRIPT_MARKER}
from __future__ import annotations

import hashlib
import json
import os
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

STATUS_PATH = Path({status_path_literal})
SOURCE = "codex-hooks"
EVENT_STATES = {{
    "UserPromptSubmit": "working",
    "PreToolUse": "working",
    "PermissionRequest": "waiting_for_confirmation",
    "PostToolUse": "working",
    "Stop": "idle",
}}
WAITING_KEYS = {{
    "approval",
    "approval_request",
    "confirmation",
    "permission",
    "permission_request",
    "request_approval",
    "requires_approval",
    "user_confirmation",
}}


def read_payload() -> dict:
    try:
        return json.load(sys.stdin)
    except Exception:
        return {{}}


def hook_event_name(payload: dict) -> str:
    if len(sys.argv) > 1 and sys.argv[1]:
        return sys.argv[1]
    for key in ("hook_event_name", "hookEventName", "event", "event_name"):
        value = payload.get(key)
        if isinstance(value, str) and value:
            return value
    return "Unknown"


def session_id(payload: dict) -> str:
    raw = (
        payload.get("session_id")
        or payload.get("sessionId")
        or payload.get("conversation_id")
        or payload.get("cwd")
        or os.getcwd()
    )
    digest = hashlib.sha256(str(raw).encode("utf-8", "replace")).hexdigest()[:16]
    return f"local-{{digest}}"


def payload_has_waiting_marker(value) -> bool:
    if isinstance(value, dict):
        for key, nested in value.items():
            if isinstance(key, str) and key.lower() in WAITING_KEYS:
                return True
            if payload_has_waiting_marker(nested):
                return True
    elif isinstance(value, list):
        return any(payload_has_waiting_marker(item) for item in value)
    return False


def state_for(event_name: str, payload: dict) -> str:
    if event_name == "PreToolUse" and payload_has_waiting_marker(payload):
        return "waiting_for_confirmation"
    return EVENT_STATES.get(event_name, "working")


def append_event(event_name: str, payload: dict) -> None:
    state = state_for(event_name, payload)
    STATUS_PATH.parent.mkdir(parents=True, exist_ok=True)
    event = {{
        "version": "1",
        "source": SOURCE,
        "session_id": session_id(payload),
        "sequence": time.time_ns(),
        "observed_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "state": state,
        "freshness": "fresh",
        "provider_metadata": {{"hook_event": event_name}},
    }}
    with STATUS_PATH.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(event, separators=(",", ":")) + "\n")
    prune_status_file()


def prune_status_file(max_lines: int = 500) -> None:
    try:
        lines = STATUS_PATH.read_text(encoding="utf-8").splitlines()
    except OSError:
        return
    if len(lines) <= max_lines:
        return
    STATUS_PATH.write_text("\n".join(lines[-max_lines:]) + "\n", encoding="utf-8")


def main() -> int:
    payload = read_payload()
    append_event(hook_event_name(payload), payload)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
"#
    )
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path).map_err(|error| format!("read hook metadata: {error}"))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).map_err(|error| format!("chmod hook script: {error}"))
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<(), String> {
    Ok(())
}
