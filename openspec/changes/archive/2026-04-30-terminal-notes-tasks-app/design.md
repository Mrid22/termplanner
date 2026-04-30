## Context

The application is a Rust TUI built with ratatui and crossterm. The UI layout is correct (20%/30%/50% three-column split), but the implementation is incomplete:

- `tasks.rs` owns `ToDoItem` and `TaskState`, which are aliased in `dates.rs` and `notes.rs` with inappropriate semantics (dates and notes are not "todo items")
- All data is hardcoded in `main.rs` at startup with no file I/O
- No date discovery or filesystem interaction
- No pane navigation (Tab/Shift+Tab)
- Single shared `TaskState` drives all three panes, making it impossible to track independent selection states
- No mechanism to open notes in an external editor

The file storage model is fixed: `~/Notes/YYYY-MM-DD/tasks.md` for tasks and `~/Notes/YYYY-MM-DD/Notes/*.md` for notes.

## Goals / Non-Goals

**Goals:**

- Implement file-based persistence matching the `~/Notes/` directory structure
- Replace the single shared `TaskState` with independent state per pane
- Add filesystem-based date discovery and loading
- Implement Tab/Shift+Tab pane navigation with scoped input
- Open notes in `$EDITOR` on selection
- Consolidate type definitions into a clean, non-duplicated model

**Non-Goals:**

- Rich text editing within the TUI (notes open in external `$EDITOR`)
- Task/note search or filtering beyond date selection
- Cross-date task aggregation or views
- Undo/redo functionality
- Configuration files or settings UI

## Decisions

### 1. State architecture: per-pane state structs instead of single shared state

**Decision**: Replace the single `TaskState` with three independent state structs (`DateState`, `TaskState`, `NoteState`) plus a top-level `AppState` that tracks the active pane and a selected date.

**Rationale**: The current approach of aliasing `TaskState` for dates and notes creates semantic confusion and couples panes together. Each pane needs its own list state, items, and input state. The `AppState` coordinates cross-pane communication (selecting a date updates tasks and notes).

**Alternatives considered**:
- Keep single state with enum-discriminated modes: too complex, doesn't match the parallel nature of the panes
- Use a single generic list state: loses type safety and clarity

### 2. Data model: new structs instead of reusing ToDoItem

**Decision**: Define purpose-built structs: `DateEntry` (date string + metadata), `Task` (description + done status), `NoteEntry` (filename + title).

**Rationale**: `ToDoItem` conflates "done" status with all items, which makes no sense for dates or notes. Purpose-built types are clearer and allow adding date-specific or note-specific fields later without breaking changes.

### 3. Task persistence: line-based markdown format

**Decision**: Store tasks as `- [ ] description` (unchecked) and `- [x] description` (checked) in `tasks.md`, one task per line.

**Rationale**: This is a widely recognized markdown task list format, human-editable, and trivially parseable. It aligns with how users expect tasks to look in markdown files.

**Alternatives considered**:
- JSON: not human-friendly for a markdown-centric workflow
- TOML: verbose for simple task lists

### 4. Note persistence: one `.md` file per note in `Notes/` subdirectory

**Decision**: Each note is a standalone `.md` file. The notes pane lists filenames (stripping `.md` extension for display). Selecting a note launches `$EDITOR` with the file path.

**Rationale**: This keeps notes as independent, portable markdown files. Opening in `$EDITOR` leverages the user's existing editor setup and avoids building a text editor into the TUI.

### 5. Date discovery: scan `~/Notes/` for directories matching `YYYY-MM-DD`

**Decision**: On startup and on refresh, scan `~/Notes/` for subdirectories matching the `YYYY-MM-DD` pattern, sort them chronologically, and populate the dates pane.

**Rationale**: Simple, deterministic, and matches the file structure requirement. No separate index file needed.

### 6. Pane focus management: `ActivePane` enum with Tab cycling

**Decision**: Add an `ActivePane` enum (`Dates | Tasks | Notes`) to `AppState`. Tab cycles forward, Shift+Tab cycles backward. Input handlers (j/k/Enter/space/D/A) are scoped to the active pane.

**Rationale**: Clear separation of concerns, easy to extend with additional pane-specific shortcuts later.

### 7. No new external dependencies

**Decision**: Use only `std::fs`, `std::env`, `std::process`, and `chrono` for date parsing. The existing dependencies (ratatui, crossterm, color-eyre) are sufficient.

**Rationale**: Keeps the binary small and build times fast. File I/O and process launching are well-supported in std.

**Alternatives considered**:
- Adding `serde`/`serde_json`: unnecessary complexity for line-based task format
- Adding `directories` crate: `~/Notes/` is explicitly specified, no need for platform-specific path resolution

## Risks / Trade-offs

- **[Risk]**: `tasks.md` parsing fails on malformed lines → **Mitigation**: Skip unparseable lines with a warning; don't crash
- **[Risk]**: `$EDITOR` not set in environment → **Mitigation**: Fall back to `vi` or display an error message
- **[Risk]**: Large number of dates or notes causes slow rendering → **Mitigation**: Acceptable for initial version; can add pagination/virtualization later
- **[Trade-off]**: No cross-date views (e.g., "all tasks this week") → **Acceptable**: Out of scope; can be added as a future enhancement
- **[Trade-off]**: No in-app note editing → **Acceptable**: Leveraging `$EDITOR` is simpler and gives users their full editor capabilities
