# Schema Validation Notes

- `status-event.v1.schema.json` is loaded and parsed by `src-tauri/tests/contract_tests.rs`.
- `display-state.v1.schema.json` is loaded and parsed by `src-tauri/tests/contract_tests.rs`.
- `validation-sequence.v1.schema.json` is loaded and parsed by `src-tauri/tests/contract_tests.rs`.
- Runtime checks enforce required fields, allowed states, stale normal-color rejection, and sensitive metadata rejection.

