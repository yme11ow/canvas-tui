use ratatui::layout::{Constraint, Direction, Layout};

pub fn build_layout(frame: &ratatui::Frame<'_>) -> Vec<ratatui::layout::Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area())
        .to_vec()
}
