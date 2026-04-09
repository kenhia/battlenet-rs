<!--
SYNC IMPACT REPORT
==================
Version change: (new) → 1.0.0
Added sections: Core Principles (I–V), Project Directory Conventions,
  Pre-Commit Standards, Governance
Removed sections: N/A (first version)
Templates requiring updates:
  ✅ .specify/templates/plan-template.md  — Constitution Check gates updated
  ✅ .specify/templates/spec-template.md  — SDD/TDD notes added
  ✅ .specify/templates/tasks-template.md — TDD task pattern added
Follow-up TODOs: none
-->

# battlenet-rs Constitution

## Core Principles

### I. Spec-Driven Development (SDD)

Every change to the codebase MUST originate from a written specification.

- Feature work MUST have a spec in `/spec/<###-feature-name>/spec.md` before
  any implementation begins.
- Ad-hoc or unplanned changes MUST be recorded in the current active spec or
  in `/spec/supplemental-spec.md` before (or immediately after, for truly
  trivial fixes) merging.
- During the polish phase of each spec/sprint, a combined specification MUST
  be written to or merged into `/docs/specification.md`.
- Implementation that cannot be traced to a spec entry is a constitution
  violation and MUST be remediated before the branch is merged.

**Rationale**: A living specification ensures design decisions are explicit,
reviewable, and recoverable — preventing the codebase from diverging from
intended behaviour over time.

### II. Test-Driven Development (TDD)

New code MUST be developed using the Red → Green → Refactor cycle.

- Tests MUST be written and confirmed to **fail** before implementation code
  is written.
- Every public API, model, and service function MUST have corresponding unit
  tests.
- Integration and contract tests MUST be added when: a new API endpoint is
  added, a data contract changes, or inter-module communication is introduced.
- Tests MUST remain passing on `main` at all times; a broken test suite blocks
  merging.

**Rationale**: Writing tests first forces a clear interface definition and
prevents regressions, which is especially critical for an API wrapper library
where contract fidelity is essential.

### III. Code Quality Standards (NON-NEGOTIABLE)

All code MUST pass the full pre-commit check suite before being committed.
The CI variant (stricter flags) MUST pass clean — this is the merge gate.

See **Pre-Commit Standards** section for per-ecosystem commands.

- Formatting violations MUST be corrected by the automated formatter (never
  suppressed via ignore comments without justification in the commit message).
- Linter warnings MUST be resolved; `#[allow(...)]` suppressions require an
  inline comment explaining why the suppression is intentional and safe.
- Code MUST compile and type-check cleanly — no `cargo check` errors.
- All existing tests MUST continue to pass after every change.

**Rationale**: Consistent style and zero-warning builds reduce cognitive load
during code review and prevent subtle bugs caused by ignored warnings.

### IV. User Documentation from the Start

Every iteration (spec/sprint) MUST update both:

- `docs/installation.md` — installation and setup guide.
- `docs/usage.md` — usage guide with current examples.

Documentation MUST be updated as part of the spec's polish phase, not
deferred to a later sprint. API changes without corresponding documentation
updates MUST NOT be merged.

**Rationale**: Library consumers (and future contributors) rely on accurate
documentation. Treating docs as a first-class deliverable prevents the
documentation debt that plagues many open-source libraries.

### V. Architecture Currency

An architecture document MUST be maintained at `docs/architecture.md`.

- MUST be created or updated during the polish phase of each spec/sprint.
- MUST be updated following any ad-hoc change that affects system structure,
  module boundaries, data flow, or external dependencies.
- MUST reflect the actual current system — aspirational or future-state
  content MUST be clearly marked as such.

**Rationale**: A current architecture document is the single source of truth
for understanding how the system fits together, enabling confident onboarding
and informed design decisions.

## Project Directory Conventions

The following directories MUST be used as indicated. Create them if absent.

| Directory       | Purpose                                                        | In `.gitignore` |
|-----------------|----------------------------------------------------------------|:---------------:|
| `.scratch-agent`| Temporary files created by the AI agent during a session       | ✅ Yes          |
| `.scratch`      | Temporary files created by the developer during exploration    | ✅ Yes          |
| `docs/`         | Project documentation (architecture, install, usage, spec)     | No              |
| `poc-ex/`       | Source for proofs-of-concept and exploratory code              | No              |
| `spec/`         | Iteration/sprint specs (one sub-directory per feature/sprint)  | No              |
| `src/`          | Production library source                                      | No              |
| `tests/`        | Integration and contract tests                                 | No              |
| `examples/`     | Runnable usage examples                                        | No              |

`.scratch-agent` and `.scratch` MUST be listed in `.gitignore`.

## Pre-Commit Standards

### Rust (primary ecosystem)

Run before every commit. All commands MUST exit 0.

```bash
# Standard (local development)
cargo fmt
cargo clippy --all-targets --all-features
cargo check
cargo test

# CI variant (merge gate — MUST pass clean)
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Python (pygen/ tooling)

See `.specify/memory/constitution.python.md` for full detail.

```bash
# CI variant summary
ruff format --check
ruff check
ty check
pytest -q
```

## Governance

- This constitution supersedes all other informal practices or conventions.
- Amendments require: (1) a written rationale, (2) version increment per
  semver rules below, (3) propagation of changes to dependent templates.
- **Versioning policy**:
  - MAJOR: backward-incompatible governance change, principle removal or
    fundamental redefinition.
  - MINOR: new principle or section added, or materially expanded guidance.
  - PATCH: clarifications, wording, or non-semantic refinements.
- All PRs MUST include a "Constitution Check" confirmation in the plan or
  task list verifying compliance with each active principle.
- Deviation from any principle MUST be explicitly justified in the PR
  description and logged in `/spec/supplemental-spec.md`.

**Version**: 1.0.0 | **Ratified**: 2026-04-07 | **Last Amended**: 2026-04-07
