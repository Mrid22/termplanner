# AGENTS.md

## Build & Run

- `cargo run` - Run the application
- `cargo build` - Build the binary
- `cargo check` or `cargo clippy` - Linting

## Dev Shell

- `nix develop` - Use the flake's dev shell (includes rustfmt, clippy,
  rust-analyzer)

## Architecture

```
src/
├── main.rs    # Entry point, app state, event loop, layout (20%/30%/50% split)
├── dates.rs   # Date list column (incomplete, has duplicate types with tasks.rs)
└── tasks.rs   # Tasks and notes columns (incomplete, duplicated from dates.rs)
```

## Current State

The app is partially scaffolded. The UI layout is correct (3-column split), but:

- `dates.rs` and `tasks.rs` contain duplicate `ToDoItem` and `TaskState`
  definitions - these need consolidation
- No actual date/task/note data loading or persistence is implemented yet

## File Storage Structure (to implement)

Per user requirements:

- All data stored under `~/Notes/`
- Date folders: `~/Notes/YYYY-MM-DD/`
- Tasks file: `~/Notes/YYYY-MM-DD/tasks.md`
- Notes folder: `~/Notes/YYYY-MM-DD/Notes/` containing `.md` files

## Key Bindings

- `q` / `Esc` - Quit
- `j` / `k` - Navigate up/down
- `Enter` / `Space` - Toggle task done
- `D` - Delete task
- `A` - Add new task
