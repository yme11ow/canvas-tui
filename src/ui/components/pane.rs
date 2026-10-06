use crate::ui::theme::BORDER_COLOR;
use ratatui::{
    prelude::*,
    widgets::{BorderType::Rounded, *},
};

pub fn pane(title: &str) -> Block<'_> {
    let border_color = BORDER_COLOR;
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(Rounded)
        .border_style(Style::default().fg(border_color))
}
