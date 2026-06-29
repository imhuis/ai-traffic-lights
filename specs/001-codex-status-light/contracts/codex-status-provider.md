# Contract: Codex Status Provider

## Purpose

The desktop app consumes sanitized status events from one provider at a time.
Providers may be live Codex integrations or deterministic validation replays.
All providers must normalize their input into
[`status-event.v1.schema.json`](status-event.v1.schema.json).

## Provider Responsibilities

- Emit status events without conversation text, shell commands, code snippets,
  file contents, approval prompt text, or secrets.
- Emit monotonically increasing `sequence` values per `session_id`.
- Surface provider health separately from normal traffic-light state.
- Map Codex activity into one of:
  - `idle`
  - `working`
  - `waiting_for_confirmation`
  - `unavailable`
- Mark events as `fresh` only while the provider transport or replay is trusted.

## Live Provider: Codex Hooks

The live provider uses user-approved Codex lifecycle hooks. The desktop app
checks for a user-level hook script and `hooks.json` entries at startup. If they
are missing, the app keeps the lamp unavailable and exposes an `Install Hooks`
action from the app tray menu. Hook installation must preserve unrelated
existing hooks.

Expected mapping:

| Codex hook | Status event |
|------------|--------------|
| `UserPromptSubmit` | `working` |
| `PreToolUse` | `working` |
| `PermissionRequest` | `waiting_for_confirmation` |
| `PostToolUse` | `working` |
| `Stop` | `idle` |
| Hook missing, untrusted, no status file, stale file, or invalid event stream | `unavailable` |

The hook script writes JSONL events to the local Traffic Lights status file with
source `codex-hooks`. It must not write prompt text, command text, file contents,
code, or secrets. Session identity must be a local hash or otherwise sanitized
identifier.

The live provider may hold a recently observed `working` or
`waiting_for_confirmation` event briefly when a `Stop` event follows before the
next UI refresh, preventing short Codex turns from being visually skipped.

Codex may require the user to run `/hooks` and review/trust non-managed hooks
before they execute.

References:

- https://developers.openai.com/codex/hooks
- https://developers.openai.com/codex/config-advanced

## Legacy Provider: Codex App Server

`codex-app-server` remains a documented experimental fallback. Local validation
on 2026-06-29 found `codex-cli 0.141.0` exposes `codex app-server` and generated
protocol schema for approval/input methods, but the current app does not open
the app-server transport stream.

## Replay Provider: JSONL

The replay provider reads one JSON object per line matching
`status-event.v1.schema.json`. It is used for tests, demos, and quickstart
validation. Replay data must be explicitly labeled as replay data and must never
be treated as live Codex status.

Example line:

```json
{"version":"1","source":"jsonl-replay","session_id":"demo-1","sequence":1,"observed_at":"2026-06-29T09:00:00Z","state":"working","freshness":"fresh"}
```

## Aggregation Contract

The app emits a display state matching
[`display-state.v1.schema.json`](display-state.v1.schema.json).

Precedence:

1. `unavailable` when provider health is not trusted.
2. `yellow` when any fresh session is waiting for confirmation.
3. `red` when no session is waiting and at least one fresh session is working.
4. `green` when all fresh known sessions are idle.
5. `unavailable` when no trusted session exists.
