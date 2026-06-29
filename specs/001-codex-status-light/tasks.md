# Tasks: Codex Status Traffic Light

**Input**: Design documents from `specs/001-codex-status-light/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required by the constitution and this feature's plan for safety
invariants, signal-decision logic, public contracts, reproducibility, privacy,
and stale-source handling. Write tests before implementation code in each user
story phase.

**Organization**: Tasks are grouped by user story so each story can be
implemented and validated as an independent increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches a different file and does not
  depend on another incomplete task.
- **[Story]**: Present only for user story tasks.
- Every task includes at least one exact file path.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize the Rust/Tauri desktop project and the minimal UI shell.

- [X] T001 Create the Rust workspace manifest with shared workspace settings in Cargo.toml
- [X] T002 Create the npm/Tauri command manifest with dev scripts in package.json
- [X] T003 Create the Tauri application manifest for the desktop app in src-tauri/Cargo.toml
- [X] T004 Create the Tauri runtime configuration for window and app metadata in src-tauri/tauri.conf.json
- [X] T005 [P] Create the UI entry document for the traffic light app in ui/index.html
- [X] T006 [P] Create Rust module declarations for domain, providers, storage, and telemetry in src-tauri/src/lib.rs
- [X] T007 [P] Create the TypeScript compiler configuration for the UI shell in tsconfig.json
- [X] T008 [P] Create ignore rules for Rust, npm, Tauri, and generated app artifacts in ./.gitignore

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish contracts, shared types, fixtures, and validation
infrastructure that every user story depends on.

**Critical**: No user story implementation begins until this phase is complete.

- [X] T009 Add Rust dependencies for Tauri 2, serde, tokio, notify, directories, tracing, and schema validation in src-tauri/Cargo.toml
- [X] T010 Add UI dependencies and npm scripts for Tauri dev/test commands in package.json
- [X] T011 [P] Create the basic replay fixture for idle, working, waiting, and idle states in src-tauri/fixtures/status-sequences/basic.jsonl
- [X] T012 [P] Create the multi-session precedence replay fixture in src-tauri/fixtures/status-sequences/multi-session-priority.jsonl
- [X] T013 [P] Create the stale-source replay fixture in src-tauri/fixtures/status-sequences/stale-source.jsonl
- [X] T014 [P] Create Rust schema loader helpers for checked-in JSON contracts in src-tauri/tests/support/schema.rs
- [X] T015 [P] Create Rust replay fixture loader helpers for JSONL status sequences in src-tauri/tests/support/replay.rs
- [X] T016 [P] Add failing contract tests for status-event, display-state, and validation-sequence schemas in src-tauri/tests/contract_tests.rs
- [X] T017 Create shared domain module exports for status, display, aggregation, and preferences in src-tauri/src/domain/mod.rs
- [X] T018 Create shared provider trait and health module exports in src-tauri/src/providers/mod.rs
- [X] T019 Create app-wide state module skeleton for display subscriptions and commands in src-tauri/src/app_state.rs

**Checkpoint**: Foundation ready; contract tests exist and fail for missing
implementation support where appropriate.

---

## Phase 3: User Story 1 - View Codex Status at a Glance (Priority: P1) MVP

**Goal**: Show a compact desktop traffic light that displays green for idle, red
for working, and returns to green when Codex is idle again.

**Independent Test**: Replay the basic scripted sequence and verify visible
color plus label match idle, working, waiting, and idle events, with red/green
transitions within 2 seconds.

### Tests for User Story 1

> Write these tests first and verify they fail before implementation.

- [X] T020 [P] [US1] Add failing aggregation tests for idle, working, and unavailable mapping in src-tauri/tests/aggregation_tests.rs
- [X] T021 [P] [US1] Add failing replay provider test for src-tauri/fixtures/status-sequences/basic.jsonl in src-tauri/tests/provider_replay_tests.rs
- [X] T022 [P] [US1] Add failing display-state command integration test for green/red responses in src-tauri/tests/app_state_tests.rs
- [X] T023 [P] [US1] Add failing desktop smoke checklist for green/red rendering and labels in tests/smoke/desktop-state-rendering.md

### Implementation for User Story 1

- [X] T024 [P] [US1] Implement CodexStatusSignal parsing and validation in src-tauri/src/domain/status_signal.rs
- [X] T025 [P] [US1] Implement TrafficLightDisplayState constructors for green, red, and unavailable in src-tauri/src/domain/display_state.rs
- [X] T026 [US1] Implement basic single-session aggregation rules in src-tauri/src/domain/aggregation.rs
- [X] T027 [US1] Implement JSONL replay provider for basic status events in src-tauri/src/providers/jsonl_replay.rs
- [X] T028 [US1] Implement display-state storage and read command in src-tauri/src/app_state.rs
- [X] T029 [US1] Wire Tauri startup, provider selection, and app state registration in src-tauri/src/main.rs
- [X] T030 [US1] Implement the UI bootstrapping and command subscription logic in ui/src/main.ts
- [X] T031 [US1] Implement the traffic-light component for green, red, and unavailable states in ui/src/traffic-light.ts
- [X] T032 [US1] Implement traffic-light layout, color, text-label, and icon styles in ui/styles/traffic-light.css

**Checkpoint**: User Story 1 works independently with the basic replay sequence
and displays idle/working/unavailable states without live Codex activity.

---

## Phase 4: User Story 2 - Notice Confirmation Requests (Priority: P2)

**Goal**: Clearly show yellow when any Codex session is waiting for user
confirmation, and keep yellow until the confirmation is resolved.

**Independent Test**: Trigger a waiting-for-confirmation replay and verify yellow
appears within 2 seconds, remains while pending, and changes to red or green only
after the pending state resolves.

### Tests for User Story 2

> Write these tests first and verify they fail before implementation.

- [X] T033 [P] [US2] Add failing aggregation tests for yellow-over-red-over-green precedence in src-tauri/tests/aggregation_tests.rs
- [X] T034 [P] [US2] Add failing replay provider test for waiting_for_confirmation transitions in src-tauri/tests/provider_replay_tests.rs
- [X] T035 [P] [US2] Add failing app-state test for yellow display fields and counts in src-tauri/tests/app_state_tests.rs
- [X] T036 [P] [US2] Add failing smoke checklist for yellow label, icon, and accessibility text in tests/smoke/desktop-state-rendering.md

### Implementation for User Story 2

- [X] T037 [US2] Extend TrafficLightDisplayState constructors and labels for yellow confirmation state in src-tauri/src/domain/display_state.rs
- [X] T038 [US2] Implement multi-session yellow-over-red-over-green precedence in src-tauri/src/domain/aggregation.rs
- [X] T039 [US2] Extend JSONL replay provider to support waiting_for_confirmation transitions in src-tauri/src/providers/jsonl_replay.rs
- [X] T040 [US2] Implement live Codex app-server provider mapping for working and waiting states in src-tauri/src/providers/codex_app_server.rs
- [X] T041 [US2] Expose provider source selection between jsonl-replay and codex-app-server in src-tauri/src/main.rs
- [X] T042 [US2] Render yellow confirmation state with distinct label and icon in ui/src/traffic-light.ts
- [X] T043 [US2] Add yellow confirmation visual styling and color-independent marker in ui/styles/traffic-light.css

**Checkpoint**: User Stories 1 and 2 both work independently; yellow takes
priority whenever confirmation is pending.

---

## Phase 5: User Story 3 - Keep the Indicator Useful as a Desktop Utility (Priority: P3)

**Goal**: Persist desktop placement and visibility preferences, and prevent stale
or unavailable status from being shown as a normal traffic-light color.

**Independent Test**: Move the indicator, save visibility preference, restart,
and verify placement is preserved. Then replay a stale-source sequence and verify
the unavailable state appears with the last known update time.

### Tests for User Story 3

> Write these tests first and verify they fail before implementation.

- [X] T044 [P] [US3] Add failing aggregation tests for stale, disconnected, invalid, and no-session display rules in src-tauri/tests/aggregation_tests.rs
- [X] T045 [P] [US3] Add failing preference persistence and visible-screen clamping tests in src-tauri/tests/preferences_tests.rs
- [X] T046 [P] [US3] Add failing privacy tests that reject persisted status payloads and sensitive telemetry in src-tauri/tests/privacy_tests.rs
- [X] T047 [P] [US3] Add failing replay provider test for src-tauri/fixtures/status-sequences/stale-source.jsonl in src-tauri/tests/provider_replay_tests.rs
- [X] T048 [P] [US3] Add failing smoke checklist for unavailable state and last-update display in tests/smoke/desktop-state-rendering.md

### Implementation for User Story 3

- [X] T049 [P] [US3] Implement UserDisplayPreference model and defaults in src-tauri/src/domain/preferences.rs
- [X] T050 [P] [US3] Implement StatusProviderHealth model and sanitized errors in src-tauri/src/providers/mod.rs
- [X] T051 [US3] Implement local JSON preference persistence in src-tauri/src/storage/preferences_store.rs
- [X] T052 [US3] Implement preference load, save, and visible-screen clamping commands in src-tauri/src/app_state.rs
- [X] T053 [US3] Implement stale, disconnected, invalid, and no-session aggregation rules in src-tauri/src/domain/aggregation.rs
- [X] T054 [US3] Implement sanitized trace events for state changes and provider health in src-tauri/src/telemetry/sanitized_events.rs
- [X] T055 [US3] Wire window placement, always-on-top, and tray visibility behavior in src-tauri/src/main.rs
- [X] T056 [US3] Render unavailable state, stale flag, and last-update text in ui/src/traffic-light.ts
- [X] T057 [US3] Add unavailable-state and compact desktop utility styling in ui/styles/traffic-light.css
- [X] T058 [US3] Add UI preference controls for visibility and compact mode in ui/src/main.ts

**Checkpoint**: All user stories are independently functional; stale or missing
status never appears as green, red, or yellow.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validate the full feature, document evidence, and prepare the app
for handoff.

- [X] T059 [P] Update the quickstart with exact verified commands and observed outcomes in specs/001-codex-status-light/quickstart.md
- [X] T060 [P] Add schema-validation notes for contract coverage in tests/contract/schema-validation.md
- [X] T061 [P] Add privacy validation evidence and sensitive-field checklist in tests/smoke/privacy-validation.md
- [X] T062 Run cargo fmt, cargo clippy, cargo test, and npm test results through the quickstart evidence section in specs/001-codex-status-light/quickstart.md
- [X] T063 Run scripted replay validation for basic, multi-session, and stale-source fixtures and record results in specs/001-codex-status-light/quickstart.md
- [X] T064 Run live Codex provider validation and record provider limitations or event-name findings in specs/001-codex-status-light/contracts/codex-status-provider.md
- [X] T065 Review all generated docs for consistency with the constitution and update the plan if implementation changed architecture in specs/001-codex-status-light/plan.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user
  story work.
- **User Story 1 (Phase 3)**: Depends on Foundational. This is the MVP.
- **User Story 2 (Phase 4)**: Depends on Foundational and can reuse US1 display
  plumbing, but its tests and yellow-precedence logic remain independently
  verifiable.
- **User Story 3 (Phase 5)**: Depends on Foundational and can be implemented
  after US1/US2 for fastest integrated validation, but preference and stale-state
  tests are independently verifiable.
- **Polish (Phase 6)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **US1**: No dependency on other stories after Foundational.
- **US2**: Depends on the shared display-state and provider interfaces from
  Foundational; reuses UI paths created by US1 if US1 was already implemented.
- **US3**: Depends on the shared app state, provider health, and UI paths from
  Foundational; stale-state validation can run without live Codex.

### Within Each User Story

- Tests must be written first and verified failing before implementation.
- Domain models before aggregation behavior.
- Aggregation behavior before provider/UI wiring.
- Replay validation before live-provider validation.
- Story checkpoint validation before moving to the next priority.

### Parallel Opportunities

- Setup tasks T005-T008 can run in parallel after T001-T004 are understood.
- Foundational fixture/helper tasks T011-T016 can run in parallel.
- US1 tests T020-T023 can run in parallel.
- US1 model/UI tasks T024, T025, and T031-T032 can run in parallel after tests.
- US2 tests T033-T036 can run in parallel.
- US3 tests T044-T048 can run in parallel.
- US3 model tasks T049-T050 can run in parallel with UI styling T056-T057 after
  tests exist.
- Polish evidence tasks T059-T061 can run in parallel before final validation
  tasks T062-T065.

---

## Parallel Examples

### User Story 1

```bash
Task: "T020 Add failing aggregation tests in src-tauri/tests/aggregation_tests.rs"
Task: "T021 Add failing replay provider test in src-tauri/tests/provider_replay_tests.rs"
Task: "T022 Add failing app-state test in src-tauri/tests/app_state_tests.rs"
Task: "T023 Add failing desktop smoke checklist in tests/smoke/desktop-state-rendering.md"
```

### User Story 2

```bash
Task: "T033 Add failing precedence tests in src-tauri/tests/aggregation_tests.rs"
Task: "T034 Add failing replay transition tests in src-tauri/tests/provider_replay_tests.rs"
Task: "T035 Add failing yellow app-state tests in src-tauri/tests/app_state_tests.rs"
Task: "T036 Add failing yellow smoke checklist in tests/smoke/desktop-state-rendering.md"
```

### User Story 3

```bash
Task: "T044 Add failing stale aggregation tests in src-tauri/tests/aggregation_tests.rs"
Task: "T045 Add failing preference tests in src-tauri/tests/preferences_tests.rs"
Task: "T046 Add failing privacy tests in src-tauri/tests/privacy_tests.rs"
Task: "T047 Add failing stale replay tests in src-tauri/tests/provider_replay_tests.rs"
Task: "T048 Add failing unavailable-state smoke checklist in tests/smoke/desktop-state-rendering.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 Setup.
2. Complete Phase 2 Foundational contracts, fixtures, helpers, and skeletons.
3. Complete Phase 3 User Story 1.
4. Stop and validate the basic JSONL replay independently.
5. Demo the compact green/red/unavailable desktop traffic light.

### Incremental Delivery

1. Add US1 for visible idle/working state from deterministic replay.
2. Add US2 for yellow confirmation priority and live-provider mapping.
3. Add US3 for persistence, stale-source protection, privacy, and utility
   behavior.
4. Run full quickstart validation after each story checkpoint.

### Parallel Team Strategy

1. One contributor sets up Tauri/Rust project files and shared test helpers.
2. One contributor writes domain and contract tests.
3. One contributor builds the minimal UI shell.
4. After Foundational, US2 and US3 tests can be drafted while US1 implementation
   proceeds, as long as same-file edits are coordinated.

## Notes

- `[P]` tasks touch different files and do not depend on incomplete same-phase
  tasks.
- `[US1]`, `[US2]`, and `[US3]` labels map directly to user stories in
  `spec.md`.
- Tests are required for this feature by the constitution and must fail before
  implementation.
- Live Codex provider work must keep sensitive Codex content out of display
  state, logs, preferences, and replay fixtures.
