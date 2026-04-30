## 1. Update task rendering

- [x] 1.1 Import `Modifier` from `ratatui::style` in `src/tasks.rs`
- [x] 1.2 Remove markdown checkbox marker (`[ ]` / `[x]`) from displayed task text in `render_tasks`
- [x] 1.3 Apply `Modifier::CROSSED_OUT` to completed task descriptions alongside existing `.dim()` styling
