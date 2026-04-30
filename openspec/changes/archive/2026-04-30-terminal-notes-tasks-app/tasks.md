## 1. Data model and state restructuring

- [x] 1.1 Define `DateEntry` struct in a shared `types.rs` or `models.rs` module (date string, display label)
- [x] 1.2 Redefine `Task` struct with `description: String` and `done: bool` fields
- [x] 1.3 Define `NoteEntry` struct with `filename: String` and `title: String` fields
- [x] 1.4 Create `DateState`, `TaskState`, `NoteState` structs each with their own `ListState`, items vector, and input state
- [x] 1.5 Create `AppState` struct holding all three pane states, `ActivePane` enum, and `selected_date: Option<String>`
- [x] 1.6 Remove old `ToDoItem` and shared `TaskState` from `tasks.rs`
- [x] 1.7 Remove type aliases (`DateItem`, `DateState`, `NoteItem`, `NotesState`) from `dates.rs` and `notes.rs`

## 2. File storage utilities

- [x] 2.1 Implement `get_notes_root()` that returns `~/Notes/` path (expanding `~`)
- [x] 2.2 Implement `ensure_notes_root()` that creates `~/Notes/` if it doesn't exist
- [x] 2.3 Implement `get_date_dir(date: &str)` that returns `~/Notes/YYYY-MM-DD/` path
- [x] 2.4 Implement `get_tasks_path(date: &str)` that returns `~/Notes/YYYY-MM-DD/tasks.md` path
- [x] 2.5 Implement `get_notes_dir(date: &str)` that returns `~/Notes/YYYY-MM-DD/Notes/` path

## 3. Date scanning and loading

- [x] 3.1 Implement `scan_dates()` that reads `~/Notes/` and returns sorted `Vec<DateEntry>` for directories matching `YYYY-MM-DD`
- [x] 3.2 Integrate `scan_dates()` into app initialization to populate `DateState`
- [x] 3.3 Update `dates.rs` rendering to display `DateEntry` items from the new state

## 4. Task loading and saving

- [x] 4.1 Implement `load_tasks(date: &str) -> Vec<Task>` that parses `tasks.md` with `- [ ]` and `- [x]` format
- [x] 4.2 Implement `save_tasks(date: &str, tasks: &[Task])` that writes tasks back to `tasks.md`
- [x] 4.3 Integrate `load_tasks()` when a date is selected to populate `TaskState`
- [x] 4.4 Update `tasks.rs` rendering to display `Task` items from the new state
- [x] 4.5 Wire task toggle (Enter/Space) to call `save_tasks()` after modifying `Task.done`
- [x] 4.6 Wire task delete (`D`) to call `save_tasks()` after removing from list
- [x] 4.7 Wire task add (`A`) to create popup, then call `save_tasks()` after adding

## 5. Note loading and editor integration

- [x] 5.1 Implement `load_notes(date: &str) -> Vec<NoteEntry>` that lists `.md` files in the date's `Notes/` directory
- [x] 5.2 Integrate `load_notes()` when a date is selected to populate `NoteState`
- [x] 5.3 Update `notes.rs` rendering to display `NoteEntry` items from the new state
- [x] 5.4 Implement `open_note_in_editor(date: &str, note: &NoteEntry)` that launches `$EDITOR` (or `vi` fallback) with the note file path
- [x] 5.5 Wire note selection (Enter) to call `open_note_in_editor()`
- [x] 5.6 Implement note creation (`A`) that prompts for name, sanitizes filename, creates `.md` file, and opens in editor

## 6. Pane navigation and input handling

- [x] 6.1 Add `ActivePane` enum (`Dates | Tasks | Notes`) to `AppState`
- [x] 6.2 Implement Tab key handler that cycles `ActivePane` forward
- [x] 6.3 Implement Shift+Tab (BackTab) key handler that cycles `ActivePane` backward
- [x] 6.4 Refactor input handling in `main.rs` to dispatch j/k/Enter/Space/D/A to the active pane's state
- [x] 6.5 Update rendering to highlight the focused pane's border with a distinct color
- [x] 6.6 Ensure q/Esc quits from any pane

## 7. Cleanup and integration

- [x] 7.1 Remove hardcoded sample data from `main.rs`
- [x] 7.2 Ensure `tasks.md` is created automatically when first task is added to a new date
- [x] 7.3 Ensure `Notes/` subdirectory is created when first note is added
- [x] 7.4 Handle edge case: gracefully handle missing `$EDITOR` environment variable
- [x] 7.5 Run `cargo clippy` and fix all warnings
- [x] 7.6 Run `cargo build` and verify no errors
