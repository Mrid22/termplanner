## Context

The Tasks pane in `src/tasks.rs` currently renders task items with visible markdown checkbox markers (`[ ]` for unchecked, `[x]` for checked). These markers are implementation details of the file format, not UI elements. The current rendering uses `format!("{} {}", marker, task.description)` which prepends the raw marker to each line. Completed tasks are only dimmed via `.dim()` styling.

## Goals / Non-Goals

**Goals:**
- Hide checkbox markers from the terminal UI while preserving them in the file format
- Add strikethrough styling to completed tasks for clearer visual distinction
- Keep file loading and saving logic unchanged

**Non-Goals:**
- Do not change the `tasks.md` file format
- Do not modify the data model (`Task` struct in `types.rs`)
- Do not change keyboard shortcuts or interaction patterns

## Decisions

**Use ratatui `Modifier::CROSSED_OUT` for completed tasks**
- ratatui supports strikethrough via `Style::add_modifier(Modifier::CROSSED_OUT)`
- This is a standard terminal escape sequence and works in most modern terminal emulators
- Alternative: prefix with a visual symbol (e.g., `~` or `✗`) — rejected because it adds visual noise, which is the problem we're solving
- Fallback: if strikethrough is unsupported, the `.dim()` styling already provides a secondary visual cue

**Strip markers at render time only**
- The `render_tasks` function in `src/tasks.rs` is the sole place where markers are displayed
- No changes needed to `load_tasks` or `save_tasks` in `src/main.rs` — they already correctly parse and write the markdown format
- The `Task` struct's `done` field already tracks state; rendering just needs to stop showing the raw marker string

## Risks / Trade-offs

- **Strikethrough terminal support** → Not all terminals render `CROSSED_OUT` modifier. Mitigation: dimmed styling remains as a secondary indicator; this is acceptable since dimming already works
- **No regression risk to file format** → Since we only change rendering, file persistence is unaffected
