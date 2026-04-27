use ratatui::widgets::ListState;

use crate::render_popup;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Widget},
};

/// Alias the shared task item/type to a date-specific name so this file's
/// identifiers match the file's purpose while still reusing the canonical
/// task types defined in `tasks.rs`.
pub type DateItem = crate::tasks::ToDoItem;
pub type DateState = crate::tasks::TaskState;

pub fn render_dates(frame: &mut Frame, app_state: &mut DateState, area: Rect) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Blue)
        .title("Dates")
        .render(area, frame.buffer_mut());
    let list = List::new(app_state.items.iter().map(|date| {
        let value = if date.is_done {
            date.description.to_span().crossed_out().dim()
        } else {
            date.description.to_span()
        };
        ListItem::from(date.description.as_str());
        ListItem::from(value)
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    if app_state.is_adding {
        render_popup(frame, app_state, String::from("Add a date"));
    }
}
