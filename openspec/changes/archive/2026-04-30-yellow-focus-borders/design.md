## Context

All three pane render functions (`render_dates`, `render_tasks`, `render_notes`) use the same pattern for focus indication: `if is_focused { Color::White } else { Color::Blue }`. This is a cross-cutting visual styling change affecting `src/dates.rs`, `src/tasks.rs`, and `src/notes.rs`.

## Goals / Non-Goals

**Goals:**
- Change focused border color to `Color::Yellow` for better visibility
- Keep text styling unchanged regardless of focus state

**Non-Goals:**
- No changes to interaction patterns, key bindings, or data models
- No changes to highlight symbol styling (already yellow)

## Decisions

**Replace `Color::White` with `Color::Yellow` for focused borders**
- Yellow is already used for the `highlight_symbol` and `highlight_style`, creating visual consistency
- White can be difficult to distinguish from default terminal text color; yellow is more distinctive

**No text color changes**
- The current code does not change text color based on focus — only the border color varies
- This remains the approach; text styling (e.g., dimmed tasks, strikethrough) is independent of pane focus

## Risks / Trade-offs

- **Yellow on certain color schemes** → Some terminal color schemes may have low contrast for yellow borders. Mitigation: unfocused blue border still provides baseline distinction; users can customize theme if needed
