use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Widget},
};

use crate::types::TaskState;

pub fn render_tasks(frame: &mut Frame, app_state: &mut TaskState, area: Rect, is_focused: bool) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    let border_color = if is_focused { Color::White } else { Color::Blue };

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(border_color)
        .title("Tasks")
        .render(area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|task| {
        let span = if task.done {
            task.description
                .clone()
                .dim()
                .add_modifier(Modifier::CROSSED_OUT)
        } else {
            task.description.clone().into()
        };
        ListItem::from(span)
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
