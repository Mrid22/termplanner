use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::{Stylize, Widget},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem},
};

use crate::types::DateState;

pub fn render_dates(frame: &mut Frame, app_state: &mut DateState, area: Rect, is_focused: bool) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    let border_color = if is_focused { Color::White } else { Color::Blue };

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(border_color)
        .title("Dates")
        .render(area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|date| {
        ListItem::from(date.date.as_str())
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
