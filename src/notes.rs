use ratatui::widgets::ListState;

use crate::render_popup;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Widget},
};

/// Alias the shared task item/type to a note-specific name so this file's
/// identifiers match the file's purpose while still reusing the canonical
/// task types defined in `tasks.rs`.
pub type NoteItem = crate::tasks::ToDoItem;
pub type NotesState = crate::tasks::TaskState;

pub fn render_notes(frame: &mut Frame, app_state: &mut NotesState, area: Rect) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Blue)
        .title("Notes")
        .render(area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|note| {
        let value = if note.is_done {
            note.description.to_span().crossed_out().dim()
        } else {
            note.description.to_span()
        };
        ListItem::from(note.description.as_str());
        ListItem::from(value)
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    if app_state.is_adding {
        render_popup(frame, app_state, String::from("Add a note"));
    }
}
