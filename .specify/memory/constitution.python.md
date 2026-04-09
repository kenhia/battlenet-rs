# battlenet-rs Constitution — Python Supplement

Applies to: `pygen/` and any other Python tooling in this repository.

## Pre-Commit Checks

All Python code MUST pass the following before commit. The CI variant MUST
pass clean as the merge gate.

```bash
# Standard (local development)
ruff format
ruff check          # optionally: ruff check --fix (user must approve --unsafe-fixes)
ty check
pytest

# CI variant (merge gate — MUST pass clean)
ruff format --check
ruff check
ty check
pytest -q
```

**IMPORTANT**: `--unsafe-fixes` (i.e. `ruff check --fix --unsafe-fixes`) MUST
be approved by the user before execution. Never apply unsafe fixes automatically.

## Tool Installation

```bash
pip install ruff          # formatter + linter
pip install ty            # Astral type checker
pip install pytest        # test runner
```

Or with a `requirements-dev.txt`:

```bash
pip install -r requirements-dev.txt
```

## Notes

- `ruff format` replaces `black` / `isort` — do not use both.
- `ty check` is the Astral type checker; do not use `mypy` alongside it without
  explicit user approval.
- Tests live under `tests/` relative to the Python package root, or alongside
  source files using `*_test.py` naming if the project uses inline tests.

**Parent constitution**: `.specify/memory/constitution.md` (version 1.0.0)
