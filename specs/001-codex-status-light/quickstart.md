# Quickstart: Codex Status Traffic Light

## Prerequisites

- Rust stable toolchain matching the plan's `Language/Version`.
- Node.js and npm for the desktop view toolchain.
- Platform desktop dependencies required by the chosen desktop shell.
- Codex installed locally for live-provider validation.

## Setup

```bash
npm install
cargo test --manifest-path src-tauri/Cargo.toml
```

Expected outcome:

- Dependency installation completes.
- Rust unit tests compile and run.
- Tests for the status aggregator initially fail until implementation is
  complete, then pass once `/speckit-implement` finishes the feature.

## Contract Validation

Validate the status and display contracts before running the app:

```bash
cargo test --manifest-path src-tauri/Cargo.toml contract
```

Expected outcome:

- `status-event.v1.schema.json` accepts valid replay events.
- Invalid events with missing `session_id`, non-monotonic sequence values, or
  sensitive payload fields are rejected.
- `display-state.v1.schema.json` rejects green/red/yellow display states when
  `is_stale=true`.

## Scripted Replay Validation

Run the desktop app against the basic replay sequence:

```bash
npm run tauri -- dev -- -- --status-source src-tauri/fixtures/status-sequences/basic.jsonl
```

Expected outcome:

- Idle event displays the green bulb without adjacent visible status copy.
- Working event displays the red bulb within 2 seconds.
- Waiting-for-confirmation event displays the yellow bulb within 2 seconds.
- Final idle event returns to green within 2 seconds.

Run the multi-session priority replay:

```bash
npm run tauri -- dev -- -- --status-source src-tauri/fixtures/status-sequences/multi-session-priority.jsonl
```

Expected outcome:

- Yellow is shown when any session is waiting for confirmation.
- Red is shown when no session is waiting and at least one session is working.
- Green is shown only when all trusted sessions are idle.

Run the stale-source replay:

```bash
npm run tauri -- dev -- -- --status-source src-tauri/fixtures/status-sequences/stale-source.jsonl
```

Expected outcome:

- The app displays the unavailable state when provider health is stale or
  disconnected.
- The app shows the last known update time.
- The app never displays stale status as green, red, or yellow.

## Live Codex Hooks Provider Validation

After scripted replay passes, run the app with the live Codex hooks provider:

```bash
npm run tauri -- dev -- -- --status-source codex-hooks
```

Expected outcome:

- If Traffic Lights hooks are missing, the app shows unavailable lamp styling and
  does not silently change `~/.codex/hooks.json`.
- When the user chooses `Install Hooks` from the app tray menu, the app writes
  `~/.codex/hooks/ai-traffic-lights-status.py` and merges Traffic Lights entries
  into `~/.codex/hooks.json` without removing existing hooks.
- If Codex requires hook review, run `/hooks` in Codex and trust the Traffic
  Lights hooks before validating live status.
- A submitted Codex prompt displays red after the hook writes a working event.
- A pending permission request displays yellow after the hook writes a
  waiting-for-confirmation event.
- If Codex emits working and stop events faster than the UI refresh interval,
  the just-observed working state remains visible briefly instead of being
  skipped.
- When Codex emits the stop hook, the app displays green.
- If hooks are missing, untrusted, or no status file exists, the app displays
  unavailable instead of the last traffic color.

Without explicit `--status-source`, the app defaults to `codex-hooks`:

```bash
npm run tauri -- dev
```

If the live lamp does not appear to react, inspect the live hooks path without
starting the desktop window:

```bash
cargo run --manifest-path src-tauri/Cargo.toml -- --print-hooks-status
cargo run --manifest-path src-tauri/Cargo.toml -- --print-display-state
```

Expected outcome:

- `--print-hooks-status` reports whether `~/.codex/hooks.json` and the Traffic
  Lights hook script are installed.
- `--print-display-state` reports the exact display color currently resolved
  from `~/.codex/ai-traffic-lights/status.jsonl`.

## Preference Persistence Validation

1. Move the indicator to a visible position.
2. Enable persistent visibility.
3. Close and reopen the app.

Expected outcome:

- The indicator reopens with the same placement and visibility preference.
- If the saved position is no longer visible, the app clamps the window to a
  visible display area.

## Privacy Validation

Run tests and inspect local app configuration after replay and live-provider
validation:

```bash
cargo test --manifest-path src-tauri/Cargo.toml privacy
```

Expected outcome:

- Preferences contain only display settings.
- Trace output includes state names, timestamps, counts, and sanitized provider
  metadata only.
- No Codex conversation text, commands, code, file contents, approval prompt
  text, or secrets are displayed or persisted.

## Implementation Validation Evidence

Verified on 2026-06-29 in this workspace:

```bash
npm install
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm test
npm run build
codex --version
codex app-server --help
codex app-server generate-json-schema --experimental --out target/codex-app-server-schema
```

Observed outcomes:

- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo test --manifest-path src-tauri/Cargo.toml` passed 22 integration tests
  across aggregation, app state, contracts, preferences, privacy, and replay
  provider behavior.
- `npm test` passed TypeScript type checking.
- `npm run build` produced `ui/dist/` assets for the Tauri `frontendDist`.
- Scripted replay validation is covered by Rust tests for `basic.jsonl`,
  `multi-session-priority.jsonl`, and `stale-source.jsonl`.
- Live Codex hooks validation covers hook installation/detection, sanitized
  status-file parsing, and tray-menu installation. Codex may still require
  `/hooks` review/trust before user-level hooks execute.
- `codex-app-server` remains an experimental fallback. `codex-cli 0.141.0`
  exposes `codex app-server`, but the app-server transport stream adapter is not
  the active live provider.
