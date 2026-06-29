# Implementation Plan: Codex Status Traffic Light

**Branch**: `` | **Date**: 2026-06-29 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-codex-status-light/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Build a compact cross-platform desktop traffic-light indicator for local Codex
activity. The app maps live Codex state into one active display state: green for
idle, red for working, yellow for waiting for confirmation, and unavailable when
the status source is stale or disconnected. Rust owns the status model,
aggregation, provider integration, persistence, observability, and validation;
the desktop shell renders the status and exposes user placement preferences.

## Technical Context

**Language/Version**: Rust 1.95.0 stable for the core app and status engine;
minimal TypeScript/HTML/CSS only for the desktop view layer

**Primary Dependencies**: Tauri 2 desktop shell; `serde`/`serde_json` for
contracts; `tokio` for async status streams; `notify` for validation-file
watching; `directories` for platform config paths; `tracing` for local
diagnostics; `schemars` or checked-in JSON Schema for contract validation

**Storage**: Local JSON preferences in the OS application config directory;
ephemeral status events from a live Codex provider or scripted JSONL source; no
conversation, command, file, code, or secret persistence

**Testing**: `cargo test` for domain and aggregation logic; integration tests
with scripted JSONL event sequences; JSON Schema contract validation; Tauri
smoke test for rendering and state transitions; manual cross-platform quickstart
validation before release

**Target Platform**: macOS, Windows, and Linux desktop environments

**Project Type**: Cross-platform desktop application

**Performance Goals**: Reflect available Codex status changes in the visible
indicator within 2 seconds for at least 95% of scripted validation events; keep
UI recognition of idle/working/waiting states under 1 second for 90% of prompts

**Constraints**: Must show exactly one display state at a time; must prioritize
waiting-for-confirmation over working and working over idle; must never treat an
unknown or stale source as green; must remain understandable without color alone;
must not store or render sensitive Codex content

**Scale/Scope**: Personal local desktop utility; supports multiple local Codex
sessions through aggregation; no remote team dashboard, cloud sync, or real-world
traffic-control behavior

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Initial Gate

- **Safety invariants**: PASS. The spec defines one active display state, a
  non-traffic unavailable state, and explicit precedence rules. Validation will
  use scripted status sequences and aggregation tests.
- **Reproducibility**: PASS. Scripted JSONL status sequences define session ID,
  state, timestamps, source, sequence number, and expected display state.
- **Test-first evidence**: PASS. Required tests cover state mapping, multi-session
  precedence, stale/unavailable handling, contract validation, and UI smoke
  recognition before implementation work changes production code.
- **Observability**: PASS. The display state exposes mapped meaning, freshness,
  last update time, provider health, and local trace events without external
  services or sensitive Codex payloads.
- **Simplicity**: PASS. The first vertical slice is one desktop app with a status
  provider interface serving two current needs: deterministic validation and live
  Codex status. No persistent service, custom model training, or distributed
  coordination is introduced.

### Post-Design Re-check

- **Safety invariants**: PASS. `data-model.md` defines the state machine and
  precedence rules; `contracts/status-event.v1.schema.json` constrains source
  events; `contracts/display-state.v1.schema.json` constrains UI state.
- **Reproducibility**: PASS. `quickstart.md` requires scripted replay for all
  states and multi-session precedence before live-provider validation.
- **Test-first evidence**: PASS. Contract, unit, integration, and smoke checks are
  identified with expected failing-first scope for `/speckit-tasks`.
- **Observability**: PASS. Display and health fields include source freshness,
  last update, unavailable reason, and sanitized provider metadata.
- **Simplicity**: PASS. The plan avoids a background cloud service and keeps the
  provider interface narrow enough to support live and scripted inputs.

## Project Structure

### Documentation (this feature)

```text
specs/001-codex-status-light/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── codex-status-provider.md
│   ├── display-state.v1.schema.json
│   ├── status-event.v1.schema.json
│   └── validation-sequence.v1.schema.json
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
package.json
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── src/
│   ├── main.rs
│   ├── app_state.rs
│   ├── domain/
│   │   ├── aggregation.rs
│   │   ├── display_state.rs
│   │   ├── preferences.rs
│   │   └── status_signal.rs
│   ├── providers/
│   │   ├── codex_app_server.rs
│   │   ├── jsonl_replay.rs
│   │   └── mod.rs
│   ├── storage/
│   │   └── preferences_store.rs
│   └── telemetry/
│       └── sanitized_events.rs
├── tests/
│   ├── aggregation_tests.rs
│   ├── contract_tests.rs
│   └── provider_replay_tests.rs
└── fixtures/
    └── status-sequences/
        ├── basic.jsonl
        ├── multi-session-priority.jsonl
        └── stale-source.jsonl
ui/
├── index.html
├── src/
│   ├── main.ts
│   └── traffic-light.ts
└── styles/
    └── traffic-light.css
tests/
├── smoke/
│   └── desktop-state-rendering.md
└── contract/
    └── schema-validation.md
```

**Structure Decision**: Use a Tauri desktop structure with Rust under
`src-tauri/` and a minimal UI shell under `ui/`. All authoritative state
transitions, aggregation, persistence, provider parsing, and validation live in
Rust. The UI receives an already-resolved display state and does not implement
Codex status rules.

## Complexity Tracking

No constitutional violations identified. The provider interface is justified
because it supports two current uses: deterministic scripted validation and live
Codex status integration. No additional service, storage engine, or distributed
architecture is planned.
