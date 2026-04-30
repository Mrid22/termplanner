## Why

Tasks in the Tasks panel currently display raw markdown checkbox markers (`[ ]` and `[x]`), which creates visual noise and detracts from the clean terminal UI. Completed tasks also lack a clear visual distinction beyond dimming, making it harder to scan the task list quickly.

## What Changes

- Remove markdown checkbox markers (`[ ]` / `[x]`) from the displayed task text in the Tasks panel
- Apply strikethrough styling to completed task descriptions for clear visual distinction
- File persistence remains unchanged — markdown checkboxes are still written to `tasks.md`

## Capabilities

### New Capabilities
- `clean-task-display`: Render tasks without markdown checkbox markers and with strikethrough for completed items in the terminal UI

### Modified Capabilities
<!-- No existing capabilities are modified; this is a new display behavior -->

## Impact

- `src/tasks.rs` — Task rendering logic (`render_tasks` function)
- No changes to file loading/saving logic in `src/main.rs`
- No changes to the underlying data model in `src/types.rs`
