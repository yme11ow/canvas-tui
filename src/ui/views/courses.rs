use ratatui::{prelude::*, widgets::*};
use crate::ui::layout::build_layout;
use crate::ui::components::pane::pane;

pub fn render(frame: &mut Frame) {
    let layout = build_layout(frame);
    let left_pane = pane("courses");
    let right_pane = pane("");

    frame.render_widget(left_pane, layout[0]);
    frame.render_widget(right_pane, layout[1]);
}