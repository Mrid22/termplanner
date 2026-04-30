use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, BorderType, Clear, Paragraph},
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod dates;
mod notes;
mod tasks;
mod types;

use dates::render_dates;
use notes::render_notes;
use tasks::render_tasks;
use types::{ActivePane, AppState, NoteEntry, Task};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

    ensure_notes_root()?;
    state.date_state.items = scan_dates()?;
    if !state.date_state.items.is_empty() {
        state.date_state.list_state.select(Some(0));
        let date = state.date_state.items[0].date.clone();
        select_date(&mut state, date)?;
    }

    ratatui::run(|terminal| run(terminal, &mut state))
}

fn get_notes_root() -> Result<PathBuf> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    Ok(PathBuf::from(home).join("Notes"))
}

fn ensure_notes_root() -> Result<()> {
    let root = get_notes_root()?;
    if !root.exists() {
        fs::create_dir_all(&root)?;
    }
    Ok(())
}

fn get_date_dir(date: &str) -> Result<PathBuf> {
    Ok(get_notes_root()?.join(date))
}

fn get_tasks_path(date: &str) -> Result<PathBuf> {
    Ok(get_date_dir(date)?.join("tasks.md"))
}

fn get_notes_dir(date: &str) -> Result<PathBuf> {
    Ok(get_date_dir(date)?.join("Notes"))
}

fn scan_dates() -> Result<Vec<types::DateEntry>> {
    let root = get_notes_root()?;
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
            && is_valid_date_format(name) {
            entries.push(types::DateEntry {
                date: name.to_string(),
            });
        }
    }

    entries.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(entries)
}

fn is_valid_date_format(s: &str) -> bool {
    if s.len() != 10 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    parts[0].len() == 4 && parts[1].len() == 2 && parts[2].len() == 2
        && parts[0].parse::<u32>().is_ok()
        && parts[1].parse::<u32>().is_ok()
        && parts[2].parse::<u32>().is_ok()
}

fn load_tasks(date: &str) -> Result<Vec<Task>> {
    let path = get_tasks_path(date)?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path)?;
    let mut tasks = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if let Some(desc) = line.strip_prefix("- [x] ") {
            tasks.push(Task {
                description: desc.to_string(),
                done: true,
            });
        } else if let Some(desc) = line.strip_prefix("- [ ] ") {
            tasks.push(Task {
                description: desc.to_string(),
                done: false,
            });
        }
    }

    Ok(tasks)
}

fn save_tasks(date: &str, tasks: &[Task]) -> Result<()> {
    let path = get_tasks_path(date)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content: String = tasks.iter().map(|t| {
        if t.done {
            format!("- [x] {}\n", t.description)
        } else {
            format!("- [ ] {}\n", t.description)
        }
    }).collect();

    fs::write(&path, content)?;
    Ok(())
}

fn load_notes(date: &str) -> Result<Vec<NoteEntry>> {
    let notes_dir = get_notes_dir(date)?;
    if !notes_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&notes_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension().and_then(|e| e.to_str())
            && ext == "md"
            && let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            let title = filename.strip_suffix(".md").unwrap_or(filename).to_string();
            entries.push(NoteEntry {
                filename: filename.to_string(),
                title,
            });
        }
    }

    entries.sort_by(|a, b| a.title.cmp(&b.title));
    Ok(entries)
}

fn select_date(state: &mut AppState, date: String) -> Result<()> {
    state.selected_date = Some(date.clone());
    state.task_state.items = load_tasks(&date)?;
    state.task_state.list_state = ratatui::widgets::ListState::default();
    if !state.task_state.items.is_empty() {
        state.task_state.list_state.select(Some(0));
    }

    state.note_state.items = load_notes(&date)?;
    state.note_state.list_state = ratatui::widgets::ListState::default();
    if !state.note_state.items.is_empty() {
        state.note_state.list_state.select(Some(0));
    }

    Ok(())
}

fn open_note_in_editor(date: &str, note: &NoteEntry) -> Result<()> {
    let notes_dir = get_notes_dir(date)?;
    let file_path = notes_dir.join(&note.filename);

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    Command::new(&editor)
        .arg(&file_path)
        .status()?;

    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn run(terminal: &mut DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.task_state.is_adding || app_state.note_state.is_adding {
                match handle_add_key_input(key, app_state) {
                    FormAction::None => {}
                    FormAction::Escape => {
                        app_state.task_state.is_adding = false;
                        app_state.task_state.task_input_val.clear();
                        app_state.note_state.is_adding = false;
                        app_state.note_state.note_input_val.clear();
                    }
                    FormAction::Submit => {
                        handle_submit_input(app_state)?;
                    }
                }
            } else {
                if handle_key_input(key, app_state)? {
                    break;
                }
            }
        }
    }
    Ok(())
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn handle_add_key_input(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        KeyCode::Esc => {
            return FormAction::Escape;
        }
        KeyCode::Enter => {
            return FormAction::Submit;
        }
        KeyCode::Char(c) => {
            if app_state.task_state.is_adding {
                app_state.task_state.task_input_val.push(c);
            } else if app_state.note_state.is_adding {
                app_state.note_state.note_input_val.push(c);
            }
        }
        KeyCode::Backspace => {
            if app_state.task_state.is_adding {
                app_state.task_state.task_input_val.pop();
            } else if app_state.note_state.is_adding {
                app_state.note_state.note_input_val.pop();
            }
        }
        _ => {}
    }
    FormAction::None
}

fn handle_submit_input(app_state: &mut AppState) -> Result<()> {
    if app_state.task_state.is_adding {
        if let Some(date) = &app_state.selected_date {
            let desc = app_state.task_state.task_input_val.clone();
            if !desc.is_empty() {
                app_state.task_state.items.push(Task {
                    description: desc,
                    done: false,
                });
                save_tasks(date, &app_state.task_state.items)?;
            }
        }
        app_state.task_state.is_adding = false;
        app_state.task_state.task_input_val.clear();
    } else if app_state.note_state.is_adding {
        if let Some(date) = &app_state.selected_date {
            let name = app_state.note_state.note_input_val.clone();
            if !name.is_empty() {
                let safe_name = sanitize_filename(&name);
                let filename = if safe_name.ends_with(".md") {
                    safe_name
                } else {
                    format!("{}.md", safe_name)
                };

                let notes_dir = get_notes_dir(date)?;
                fs::create_dir_all(&notes_dir)?;
                let file_path = notes_dir.join(&filename);
                fs::write(&file_path, "")?;

                app_state.note_state.items.push(NoteEntry {
                    filename: filename.clone(),
                    title: name.clone(),
                });

                let note = NoteEntry { filename, title: name };
                open_note_in_editor(date, &note)?;
            }
        }
        app_state.note_state.is_adding = false;
        app_state.note_state.note_input_val.clear();
    }
    Ok(())
}

fn handle_key_input(key: KeyEvent, app_state: &mut AppState) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            return Ok(true);
        }
        KeyCode::Tab => {
            app_state.active_pane = app_state.active_pane.next();
            return Ok(false);
        }
        KeyCode::BackTab => {
            app_state.active_pane = app_state.active_pane.prev();
            return Ok(false);
        }
        _ => {}
    }

    match app_state.active_pane {
        ActivePane::Dates => {
            handle_dates_input(key, app_state)?;
        }
        ActivePane::Tasks => {
            handle_tasks_input(key, app_state)?;
        }
        ActivePane::Notes => {
            handle_notes_input(key, app_state)?;
        }
    }

    Ok(false)
}

fn handle_dates_input(key: KeyEvent, app_state: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.date_state.list_state.select_next();
            if let Some(idx) = app_state.date_state.list_state.selected()
                && let Some(date_entry) = app_state.date_state.items.get(idx) {
                select_date(app_state, date_entry.date.clone())?;
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.date_state.list_state.select_previous();
            if let Some(idx) = app_state.date_state.list_state.selected()
                && let Some(date_entry) = app_state.date_state.items.get(idx) {
                select_date(app_state, date_entry.date.clone())?;
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(idx) = app_state.date_state.list_state.selected()
                && let Some(date_entry) = app_state.date_state.items.get(idx) {
                select_date(app_state, date_entry.date.clone())?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_tasks_input(key: KeyEvent, app_state: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.task_state.list_state.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.task_state.list_state.select_previous();
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(idx) = app_state.task_state.list_state.selected()
                && let Some(task) = app_state.task_state.items.get_mut(idx) {
                task.done = !task.done;
                if let Some(date) = &app_state.selected_date {
                    save_tasks(date, &app_state.task_state.items)?;
                }
            }
        }
        KeyCode::Char('D') => {
            if let Some(idx) = app_state.task_state.list_state.selected() {
                app_state.task_state.items.remove(idx);
                if let Some(date) = &app_state.selected_date {
                    save_tasks(date, &app_state.task_state.items)?;
                }
                if app_state.task_state.items.is_empty() {
                    app_state.task_state.list_state.select(None);
                } else if idx >= app_state.task_state.items.len() {
                    app_state.task_state.list_state.select(Some(app_state.task_state.items.len() - 1));
                } else {
                    app_state.task_state.list_state.select(Some(idx));
                }
            }
        }
        KeyCode::Char('A') => {
            app_state.task_state.is_adding = true;
            app_state.task_state.task_input_val.clear();
        }
        _ => {}
    }
    Ok(())
}

fn handle_notes_input(key: KeyEvent, app_state: &mut AppState) -> Result<()> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.note_state.list_state.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.note_state.list_state.select_previous();
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(idx) = app_state.note_state.list_state.selected()
                && let Some(note) = app_state.note_state.items.get(idx)
                && let Some(date) = &app_state.selected_date {
                open_note_in_editor(date, note)?;
            }
        }
        KeyCode::Char('A') => {
            app_state.note_state.is_adding = true;
            app_state.note_state.note_input_val.clear();
        }
        _ => {}
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let border_area = Layout::horizontal(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
    ])
    .margin(1)
    .split(frame.area());

    render_dates(frame, &mut app_state.date_state, border_area[0], app_state.active_pane == ActivePane::Dates);
    render_tasks(frame, &mut app_state.task_state, border_area[1], app_state.active_pane == ActivePane::Tasks);
    render_notes(frame, &mut app_state.note_state, border_area[2], app_state.active_pane == ActivePane::Notes);

    if app_state.task_state.is_adding {
        render_popup(frame, &app_state.task_state.task_input_val, String::from("Add a task"));
    }
    if app_state.note_state.is_adding {
        render_popup(frame, &app_state.note_state.note_input_val, String::from("Add a note"));
    }
}

pub fn render_popup(frame: &mut Frame, input_val: &str, title: String) {
    let popup_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Center);
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph = Paragraph::new(input_val).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}
