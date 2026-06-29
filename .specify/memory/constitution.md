<!--
Sync Impact Report
Version change: template placeholders -> 1.0.0
Modified principles:
- Template PRINCIPLE_1 -> I. Safety Invariants Are Non-Negotiable
- Template PRINCIPLE_2 -> II. Reproducible Simulation and Data Provenance
- Template PRINCIPLE_3 -> III. Test-First Traffic Behavior
- Template PRINCIPLE_4 -> IV. Observable and Explainable Decisions
- Template PRINCIPLE_5 -> V. Simple, Incremental Architecture
Added sections:
- Traffic Domain Constraints
- Development Workflow
Removed sections:
- None
Templates requiring updates:
- ✅ updated .specify/templates/plan-template.md
- ✅ updated .specify/templates/spec-template.md
- ✅ updated .specify/templates/tasks-template.md
- ✅ reviewed .specify/extensions/agent-context/commands/speckit.agent-context.update.md
- ✅ reviewed AGENTS.md
- ✅ reviewed .specify/extensions/agent-context/README.md
Follow-up TODOs:
- None
-->
# AI Traffic Lights Constitution

## Core Principles

### I. Safety Invariants Are Non-Negotiable
Every feature that affects signal timing, phase transitions, simulated road state,
or traffic-rule behavior MUST define its safety invariants before implementation.
At minimum, applicable features MUST state how they prevent conflicting green
movements, invalid phase transitions, unbounded starvation, and violations of
pedestrian or emergency-priority assumptions. These invariants MUST be traceable
to acceptance scenarios, tests, contracts, or reproducible simulation checks.

Rationale: Traffic-control behavior can optimize throughput only after the
project proves that unsafe states are excluded.

### II. Reproducible Simulation and Data Provenance
Simulation, AI, heuristic, data import, and evaluation behavior MUST be
reproducible from documented inputs. Plans MUST record scenario sources, random
seeds, model or algorithm versions, units, time windows, and external data
dependencies. Generated metrics or recommendations MUST NOT rely on untracked
runtime state.

Rationale: A traffic-light decision cannot be debugged, compared, or trusted
unless another contributor can replay the same scenario and inspect the same
inputs.

### III. Test-First Traffic Behavior
Each user story MUST have an independent validation path before implementation
starts. Tests for safety invariants, signal-decision logic, public contracts,
schema changes, and reproducibility guarantees MUST be written first and MUST
fail for the missing behavior before production code is changed. Non-code
changes MUST include an equivalent checklist, scenario replay, or documented
manual verification path.

Rationale: The highest-risk behavior in this project is domain logic that appears
plausible but fails under specific traffic states.

### IV. Observable and Explainable Decisions
Features that make, recommend, simulate, or evaluate signal decisions MUST expose
the relevant inputs, selected action, rejected constraints, scoring basis or
confidence when available, and timing metrics through logs, telemetry, debug
output, or generated reports. Observability MUST avoid leaking sensitive data and
MUST be usable in local development without external services.

Rationale: AI and heuristic decisions need an audit trail so failures can be
diagnosed from scenario evidence rather than guessed from final outcomes.

### V. Simple, Incremental Architecture
Each feature MUST deliver the smallest independently useful vertical slice.
Plans MUST justify new services, persistent storage, custom AI model training,
distributed coordination, or non-trivial abstractions in Complexity Tracking and
MUST document the simpler alternative that was rejected. Shared abstractions MUST
serve at least two current use cases or remove demonstrated duplication.

Rationale: The project is early enough that excess architecture would obscure
traffic behavior, slow validation, and make safety evidence harder to review.

## Traffic Domain Constraints

- Signal phases MUST be modeled as explicit states and valid transitions, not as
  ad hoc strings or unvalidated numeric codes.
- Time units, coordinate systems, road topology assumptions, lane definitions,
  and queue-length semantics MUST be documented wherever they affect behavior.
- Human safety, pedestrian priority, emergency priority, and legal or regulatory
  assumptions MUST be represented as constraints before they are used as scoring
  factors.
- Simulation results MUST be labeled as simulation results. Any claim about
  real-world deployment readiness MUST cite the validation environment and the
  evidence used to support that claim.
- Scenario fixtures and generated data MUST be small enough for routine local
  validation unless the plan documents why larger artifacts are required.

## Development Workflow

- Every feature MUST start from `spec.md`, proceed through `plan.md`, and produce
  `tasks.md` before implementation work begins.
- The Constitution Check in `plan.md` MUST pass before Phase 0 research and MUST
  be re-checked after Phase 1 design. Any violation MUST be recorded in
  Complexity Tracking with a mitigation or rejection rationale.
- Tasks MUST be grouped by independently testable user story. Cross-story
  dependencies MUST be limited to foundational setup or explicitly documented.
- Implementation work MUST include a validation artifact: automated tests,
  reproducible scenario replay, quickstart evidence, or a manual checklist when
  automation is not practical.
- Documentation updates MUST accompany changes to domain assumptions, safety
  invariants, scenario data, model behavior, or operational commands.

## Governance

This constitution supersedes conflicting project practices and template guidance.
Amendments require a documented rationale, an explicit semantic-version bump, a
summary of affected templates or guidance files, and a migration note for active
features when existing plans or tasks are impacted.

Versioning follows semantic versioning:
- MAJOR for removing or redefining principles in a backward-incompatible way.
- MINOR for adding principles, sections, or materially expanded governance.
- PATCH for clarifications, wording fixes, and non-semantic refinements.

Compliance review is required at feature planning, after design, and before work
is considered complete. Reviewers MUST verify safety invariants, reproducibility
inputs, test-first evidence, observability, and complexity justifications. Any
approved exception MUST be recorded in the feature plan and revisited before
release or demo.

**Version**: 1.0.0 | **Ratified**: 2026-06-29 | **Last Amended**: 2026-06-29
