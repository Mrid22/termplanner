## Why

The current application has the correct UI layout (3-column split at 20%/30%/50%) but lacks functional data persistence and proper date-task-note relationships. All data is hardcoded at startup with no file I/O, duplicate type definitions exist across modules, and there is no way to navigate between panes or open notes in an external editor. This change completes the application by implementing file-based storage, proper date selection, and full keyboard navigation.

## What Changes

- Implement file-based data persistence under `~/Notes/` with date-structured folders
- Consolidate duplicate type definitions (`ToDoItem`, `TaskState`) into a single shared module
- Add date scanning and loading from the filesystem on startup
- Link date selection to tasks and notes display (filtering by selected date)
- Implement Tab/Shift+Tab navigation between the three panes (Dates, Tasks, Notes)
- Add note opening in `$EDITOR` when selecting a note item
- Implement task loading/saving from `tasks.md` per date
- Implement note listing from per-date `Notes/` subdirectories
- Add proper keyboard input handling scoped to the active pane

## Capabilities

### New Capabilities

- `file-storage`: File-based persistence under `~/Notes/YYYY-MM-DD/` with `tasks.md` and `Notes/` subdirectory structure
- `date-navigation`: Scanning, listing, and selecting dates from the filesystem
- `pane-navigation`: Tab/Shift+Tab navigation between Dates, Tasks, and Notes panes with scoped input handling
- `task-management`: Loading, saving, toggling, adding, and deleting tasks per date in markdown format
- `note-management`: Listing notes per date and opening selected notes in `$EDITOR`

### Modified Capabilities

<!-- No existing capabilities to modify; this is a net-new implementation -->

## Impact

- **Affected code**: `src/main.rs` (event loop, state management), `src/tasks.rs` (shared types, task I/O), `src/dates.rs` (date scanning, rendering), `src/notes.rs` (note listing, editor launch)
- **New dependencies**: None required (using std::fs, std::env, std::process)
- **Breaking changes**: None - this completes an incomplete scaffold
