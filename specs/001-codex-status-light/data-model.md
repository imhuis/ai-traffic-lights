# Data Model: Codex Status Traffic Light

## Overview

The domain model separates raw provider events from user-visible display state.
Provider events may arrive from live Codex integration or scripted validation,
but only sanitized, normalized status signals reach the aggregator.

## Entity: CodexStatusSignal

Represents the latest known state for one local Codex session.

### Fields

- `session_id`: Stable local identifier for the Codex session. Required.
- `source`: Provider identifier, such as `codex-app-server` or `jsonl-replay`.
  Required.
- `sequence`: Monotonic provider sequence number for the session. Required.
- `observed_at`: Timestamp when the provider observed the state. Required.
- `received_at`: Timestamp when the app accepted the state. Required.
- `state`: One of `idle`, `working`, `waiting_for_confirmation`, `unavailable`.
  Required.
- `freshness`: One of `fresh`, `stale`, `disconnected`, `invalid`. Required.
- `reason_code`: Sanitized reason for unavailable or invalid state. Optional.
- `provider_metadata`: Sanitized metadata such as provider version or transport
  name. Optional; must not include conversation, command, code, file, or secret
  contents.

### Validation Rules

- `session_id` must be non-empty and must not contain path separators.
- `sequence` must increase for each session and provider.
- `observed_at` must not be later than `received_at` by more than tolerated clock
  skew.
- `state=waiting_for_confirmation` must not include prompt text or command
  content.
- `freshness=fresh` is allowed only when provider health is currently trusted.

## Entity: TrafficLightDisplayState

Represents exactly what the desktop UI displays.

### Fields

- `display`: One of `green`, `red`, `yellow`, `unavailable`. Required.
- `label`: Short state label, such as `Idle`, `Working`, `Needs confirmation`,
  or `Unavailable`. It may be exposed through accessibility or tooltips rather
  than adjacent visible copy. Required.
- `accessibility_label`: Full label for assistive technologies. Required.
- `mapped_meaning`: User-facing explanation of what the state means. Required.
- `last_update`: Latest accepted signal timestamp used for the display. Required
  except during first-launch unavailable state.
- `is_stale`: Boolean indicating whether the display is based on untrusted or
  expired provider data. Required.
- `active_session_count`: Number of fresh known sessions represented. Required.
- `waiting_session_count`: Number of fresh sessions waiting for confirmation.
  Required.
- `working_session_count`: Number of fresh sessions working. Required.
- `unavailable_reason`: Sanitized reason when `display=unavailable`. Optional.

### Validation Rules

- Exactly one `display` value is active at a time.
- `display=yellow` if `waiting_session_count > 0`.
- `display=red` if `waiting_session_count = 0` and `working_session_count > 0`.
- `display=green` only if at least one trusted session is known, every trusted
  session is idle, and `is_stale=false`.
- `display=unavailable` when no trusted session is known, provider health is
  disconnected, or accepted events fail validation.
- `label` and `accessibility_label` must be present for every display value.

## Entity: UserDisplayPreference

Represents user-controlled desktop presentation choices.

### Fields

- `window_position`: Last known screen and coordinates. Optional at first launch.
- `window_size`: Last known window size. Optional at first launch.
- `always_on_top`: Whether the indicator should stay above normal windows.
  Required.
- `compact_mode`: Whether to show a compact indicator layout. Required.
- `show_in_tray`: Whether tray access is enabled when supported. Required.
- `launch_at_login`: Whether the app should open automatically after login.
  Optional for MVP.
- `updated_at`: Timestamp when preferences were last saved. Required.

### Validation Rules

- Window position must be clamped to a visible display area on startup.
- Preferences must not contain Codex status payloads or history.
- Missing preferences use safe defaults: visible window, normal layout, tray
  enabled where supported, and no launch-at-login unless enabled by the user.

## Entity: StatusProviderHealth

Represents whether a status provider can currently be trusted.

### Fields

- `provider`: Provider identifier. Required.
- `status`: One of `connected`, `disconnected`, `degraded`, `replaying`,
  `invalid`. Required.
- `last_heartbeat`: Timestamp of last provider health signal. Optional.
- `last_error_code`: Sanitized error code. Optional.
- `last_error_message`: Sanitized user-safe explanation. Optional.

### Validation Rules

- Provider errors must not include raw Codex prompts, command arguments, file
  contents, or secrets.
- `status=connected` is required before new live signals can be treated as fresh.
- Replayed validation files must use `status=replaying` and must not be confused
  with live Codex state.

## State Transitions

### Session State

```text
unavailable -> idle
unavailable -> working
unavailable -> waiting_for_confirmation
idle -> working
working -> waiting_for_confirmation
waiting_for_confirmation -> working
waiting_for_confirmation -> idle
working -> idle
any state -> unavailable
```

Invalid transitions are accepted only as unavailable provider errors. They must
not produce green, red, or yellow display state until a fresh trusted signal is
received.

### Aggregated Display State

```text
if provider untrusted: unavailable
else if any session waiting_for_confirmation: yellow
else if any session working: red
else if all trusted sessions idle: green
else: unavailable
```

## Privacy Constraints

- The model must not include conversation text, shell commands, file paths, code
  snippets, approval prompt text, or secrets.
- Debug output may include state names, timestamps, counts, provider names, and
  sanitized error codes only.
