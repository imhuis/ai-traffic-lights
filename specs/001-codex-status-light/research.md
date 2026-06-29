# Research: Codex Status Traffic Light

## Decision: Use Tauri 2 as the desktop shell with Rust-owned status logic

**Rationale**: Tauri is an official Rust-centered cross-platform desktop toolkit
with documented support for desktop windows, commands, and app integrations. It
lets the project keep status aggregation, persistence, provider parsing, and
telemetry in Rust while using a small view layer for the traffic-light display.
Its tray/window capabilities fit a compact always-visible desktop utility.

**Alternatives considered**:

- **egui/eframe**: Strong pure-Rust UI option, but less direct for native tray
  and desktop-app packaging expectations across macOS, Windows, and Linux.
- **Slint**: Good native UI candidate, but Tauri has broader app-shell and
  desktop integration documentation for this feature's tray/window needs.
- **Electron**: Mature desktop shell, but it would move too much of the app away
  from the user's Rust preference and increase runtime footprint.

References:
- https://v2.tauri.app/start/
- https://v2.tauri.app/develop/
- https://v2.tauri.app/develop/system-tray/

## Decision: Prefer a live Codex app-server provider plus a scripted JSONL provider

**Rationale**: Official Codex app-server documentation describes a local
integration surface for rich clients, including streamed agent events and
approval flows. That is the most appropriate live-provider target for mapping
working and waiting-for-confirmation states. A scripted JSONL provider is still
required because the constitution requires reproducible validation independent
of live Codex activity.

The provider interface is intentionally narrow: it emits sanitized status events
that contain session identity, state, timestamp, sequence number, freshness, and
provider health. It must not pass conversation text, commands, file contents, or
secrets to the UI.

**Alternatives considered**:

- **Scrape terminal or IDE UI**: Rejected because it is brittle, hard to test,
  and likely to capture sensitive content.
- **Process polling only**: Rejected because a running process does not
  distinguish working, waiting for confirmation, and idle.
- **JSONL provider only**: Useful for validation, but it does not by itself prove
  live Codex integration.
- **Cloud dashboard**: Rejected as outside the personal local desktop scope.

Reference:
- https://developers.openai.com/codex/app-server

## Decision: Aggregate multiple sessions with yellow > red > green precedence

**Rationale**: Waiting for confirmation is the only state that requires immediate
human action. If any known session is waiting for confirmation, the display must
be yellow. If none are waiting and at least one is working, it must be red. Green
is valid only when every known fresh session is idle. Unknown, disconnected, or
stale status must display unavailable rather than a normal traffic color.

**Alternatives considered**:

- **Last-event-wins**: Rejected because a later idle event from one session could
  hide a pending confirmation in another session.
- **Per-session lights only**: Deferred because it complicates the MVP. The
  display state can include session counts for later expansion.

## Decision: Treat provider freshness as part of the display state

**Rationale**: The spec forbids showing stale information as green, red, or
yellow. Live providers must surface connection health, and replay providers must
include event timestamps. The app marks status unavailable when the provider is
disconnected, the event stream violates ordering, or freshness policy says the
state can no longer be trusted.

**Alternatives considered**:

- **Keep last known color forever**: Rejected because it can mislead the user
  after the status source dies.
- **Force stale after a fixed idle duration only**: Rejected because idle can be a
  legitimate long-lived state when provider health is still known.

## Decision: Store only display preferences locally

**Rationale**: The app only needs durable window placement, visibility, compact
mode, and optional launch preference. Status events are transient and sensitive
payloads are prohibited. This satisfies the feature while keeping storage small
and auditable.

**Alternatives considered**:

- **Persist status history**: Rejected for MVP because it increases privacy risk
  and is not required for the visible indicator.
- **No persistence**: Rejected because user placement and visibility preference
  are explicit requirements.

## Decision: Use test-first validation around contracts and state transitions

**Rationale**: The risky behavior is not UI rendering; it is incorrect mapping
from Codex signals to user-visible meaning. Unit tests must cover state
aggregation and stale handling. Contract tests must validate accepted provider
events. Smoke tests must verify color, label/icon, and unavailable state
rendering.

**Alternatives considered**:

- **Manual visual checks only**: Rejected because they cannot prove precedence,
  stale handling, or privacy constraints.
- **End-to-end desktop tests only**: Rejected because they are slower and less
  precise than testing the Rust state engine directly.
