## Why

The focused panel border currently uses `Color::White`, which is visually subtle and doesn't provide a clear indication of which panel is active. Using `Color::Yellow` — already the highlight color for the selection symbol — creates a consistent and more visible focus indicator across the UI.

## What Changes

- Change focused panel border color from `Color::White` to `Color::Yellow`
- Remove any text color changes tied to focus state; text styling remains unchanged regardless of focus

## Capabilities

### New Capabilities

### Modified Capabilities
- `pane-navigation`: Focus indicator styling changes from white border to yellow border; text color no longer changes with focus state

## Impact

- `src/dates.rs` — border color in `render_dates`
- `src/tasks.rs` — border color in `render_tasks`
- `src/notes.rs` — border color in `render_notes`
