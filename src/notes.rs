use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::{Stylize, Widget},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem},
};

use crate::types::NoteState;

pub fn render_notes(frame: &mut Frame, app_state: &mut NoteState, area: Rect, is_focused: bool) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    let border_color = if is_focused { Color::Yellow } else { Color::Blue };

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title("Notes")
        .render(area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|note| {
        ListItem::from(note.title.as_str())
    }))
    .style(Style::default().fg(Color::Blue))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
