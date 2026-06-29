# Feature Specification: Codex Status Traffic Light

**Feature Branch**: `001-codex-status-light`

**Created**: 2026-06-29

**Status**: Draft

**Input**: User description: "构建一个跨平台桌面 AI 交通灯，用红黄绿显示 Codex 工作状态。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Codex Status at a Glance (Priority: P1)

A user keeps a compact desktop traffic light visible while working. The light
shows green when Codex is idle, red when Codex is actively working, and yellow
when Codex is waiting for user confirmation.

**Why this priority**: This is the core value of the app: the user can
understand Codex's current state without switching windows or reading logs.

**Independent Test**: Play a scripted sequence of idle, working, waiting for
confirmation, and idle states. Verify the active bulb position and color match
every state in the sequence without showing an adjacent status panel.

**Acceptance Scenarios**:

1. **Given** Codex is idle, **When** the app receives the idle state, **Then** the
   traffic light displays the green bulb without adjacent status copy.
2. **Given** Codex starts working, **When** the status changes from idle to
   working, **Then** the traffic light displays red within 2 seconds.
3. **Given** Codex finishes work and no confirmation is pending, **When** the
   status returns to idle, **Then** the traffic light displays green within
   2 seconds.

---

### User Story 2 - Notice Confirmation Requests (Priority: P2)

A user can immediately notice when Codex is blocked on a confirmation request so
they can respond before work stalls for too long.

**Why this priority**: Waiting for confirmation is the state that most needs
human attention; showing it clearly reduces lost time.

**Independent Test**: Trigger a waiting-for-confirmation state and verify the
indicator turns yellow, remains yellow while confirmation is pending, and leaves
yellow only after the pending confirmation is resolved or cancelled.

**Acceptance Scenarios**:

1. **Given** Codex is waiting for confirmation, **When** the app receives that
   state, **Then** the traffic light displays yellow within 2 seconds.
2. **Given** the traffic light is yellow, **When** Codex resumes work after the
   user confirms, **Then** the traffic light changes to red within 2 seconds.
3. **Given** a user cannot rely on color alone, **When** any state is displayed,
   **Then** the active bulb position and accessibility label distinguish the
   state without requiring adjacent visible text.

---

### User Story 3 - Keep the Indicator Useful as a Desktop Utility (Priority: P3)

A user can place the traffic light where it is convenient, keep it visible during
work, and rely on the app to avoid showing stale or misleading status.

**Why this priority**: The app must stay useful during daily work, but layout and
stale-status handling are secondary to the core color-state mapping.

**Independent Test**: Move the indicator, change its visibility preference,
restart the app, and verify the placement and preference are preserved. Then
simulate a missing status source and verify the app shows an unavailable state
instead of a normal green, yellow, or red state.

**Acceptance Scenarios**:

1. **Given** the user moves the indicator and enables persistent visibility,
   **When** the app restarts, **Then** the indicator reopens with the same
   placement and visibility preference.
2. **Given** Codex status cannot be determined, **When** the status is stale or
   unavailable, **Then** the app displays a clearly distinct unavailable state
   with the last known update time.
3. **Given** the user starts the app and Codex hooks are not installed, **When**
   the app checks the live status source, **Then** it keeps the lamp in an
   unavailable visual state and exposes hook installation from the app menu
   instead of silently modifying Codex configuration.
4. **Given** the user confirms hook installation, **When** the app writes the
   hook script and hooks configuration, **Then** existing Codex hooks remain
   intact and the app tells the user that Codex may still require `/hooks`
   review/trust before hooks run.

### Edge Cases

- Codex changes states rapidly several times in a short period; the app displays
  the latest known state without flicker that hides the final state.
- Codex status is unavailable at launch; the app shows unavailable status and
  does not imply Codex is idle.
- Codex reports a waiting-for-confirmation state while other work is active; the
  app prioritizes yellow so the user sees that action is needed.
- Multiple local Codex sessions are active; yellow is shown if any session is
  waiting for confirmation, red if none are waiting and at least one is working,
  and green only when all known sessions are idle.
- The app loses access to status updates after showing a valid state; the app
  marks the state stale after a documented timeout and shows the last update
  time.
- The user views the indicator on a small display or high-DPI display; the light
  remains legible and does not require reading small text to know the state.
- Codex hooks are installed but not yet trusted by Codex; the app keeps showing
  unavailable or no-status guidance until Codex runs the hook and writes a fresh
  status event.
- A user already has custom Codex hooks; installing Traffic Lights hooks appends
  only the required hook commands and must not remove or rewrite unrelated hook
  entries.

### Safety & Domain Constraints *(mandatory when feature affects traffic behavior)*

- **SI-001**: The app MUST define exactly one active display state at a time:
  green for idle, red for working, yellow for waiting for confirmation, or a
  distinct unavailable state when Codex status is not trustworthy.
- **SI-002**: The app MUST NOT present itself as a real-world traffic-control,
  pedestrian-safety, or transportation signal system.
- **RP-001**: Status behavior MUST be reproducible from a scripted state sequence
  containing state name, timestamp, session identity, and expected display state.
- **EX-001**: The app MUST expose the current state, mapped meaning, last update
  time, and whether the state is fresh or stale.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The app MUST provide a desktop-visible traffic-light indicator with
  three normal states: green for idle, red for working, and yellow for waiting
  for confirmation.
- **FR-002**: The app MUST update the visible indicator within 2 seconds after a
  Codex status change is available to the app.
- **FR-003**: The app MUST keep the lamp understandable without relying on color
  alone by using fixed bulb position, unavailable patterning, and accessibility
  labels. It MUST NOT require an adjacent visible status text panel.
- **FR-004**: The app MUST prioritize waiting-for-confirmation over working when
  more than one Codex session or state is present.
- **FR-005**: The app MUST show green only when every known Codex session is idle
  and no confirmation is pending.
- **FR-006**: The app MUST show a distinct unavailable or stale state when it
  cannot determine current Codex status.
- **FR-007**: The app MUST show the last known update time when status is stale
  or unavailable.
- **FR-008**: The user MUST be able to move the indicator and keep it visible
  while using other desktop applications.
- **FR-009**: The app MUST remember the user's indicator placement and visibility
  preference between launches.
- **FR-010**: The app MUST support the user's main desktop environments on
  macOS, Windows, and Linux.
- **FR-011**: The app MUST avoid displaying or storing Codex conversation,
  command, code, file, or secret contents as part of the status indicator.
- **FR-012**: The app MUST provide a validation mode or scripted sequence that
  demonstrates each supported state without requiring live Codex activity.
- **FR-013**: The app MUST support a live `codex-hooks` status source that maps
  Codex lifecycle hooks into sanitized local status events.
- **FR-014**: On startup, when `codex-hooks` is the active source and required
  hooks are missing, the app MUST expose an explicit app menu action to install
  them instead of silently modifying user-level Codex configuration.
- **FR-015**: Hook installation MUST preserve existing Codex hooks and add only
  Traffic Lights hook commands for the required lifecycle events.
- **FR-016**: Hook scripts MUST write only sanitized state metadata: state name,
  timestamp, source, session identity hash, sequence, freshness, and hook event
  name. They MUST NOT write prompt text, shell commands, file contents, code, or
  secrets.
- **FR-017**: The live hooks provider MUST keep a just-observed working or
  waiting-for-confirmation event visible briefly when a fast `Stop` event arrives
  before the UI's next refresh, so short Codex turns are not skipped visually.

### Key Entities *(include if feature involves data)*

- **Codex Status Signal**: Represents the latest known Codex state for one local
  session, including state name, session identity, timestamp, freshness, and
  source availability.
- **Traffic Light Display State**: Represents the visual state shown to the user,
  including color, label or icon, mapped meaning, and last update time.
- **User Display Preference**: Represents user-controlled desktop placement and
  visibility choices that persist between launches.
- **Codex Hook Installation**: Represents whether the user-level Codex hook
  script and `hooks.json` entries needed by the live provider are present, which
  lifecycle events are configured, and whether the user should review/trust them
  in Codex.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a scripted local validation sequence, 95% of status changes are
  reflected in the visible indicator within 2 seconds.
- **SC-002**: In a recognition test, users correctly identify idle, working, and
  waiting-for-confirmation states within 1 second for at least 90% of prompts.
- **SC-003**: The app remains usable for an 8-hour work session without showing a
  stale state as green, red, or yellow after status updates stop.
- **SC-004**: The same feature behavior is available on macOS, Windows, and
  Linux desktop environments.
- **SC-005**: During validation, no Codex conversation, command, code, file, or
  secret contents are displayed in the indicator or persisted as preferences.

## Assumptions

- The user is the primary audience and runs the indicator locally on the same
  machine where they use Codex.
- A local source of Codex status is available or can be provided during
  implementation planning. For live status, the preferred source is user-approved
  Codex hooks that write sanitized local status events.
- The initial feature is a personal desktop utility, not a team dashboard,
  remote monitoring tool, or real traffic-control system.
- The app may include a non-traffic unavailable state for stale or missing
  status because treating unknown status as green would be misleading.
- The indicator's visual design can be compact, but it must remain understandable
  without color alone.
