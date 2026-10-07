use crate::api::models::TabKind;
use crate::app::App;
use crate::ui::components::pane::pane;
use crate::ui::layout::build_layout;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::widgets::Block;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = build_layout(frame);
    app.courses.render(frame, layout[0], pane("courses"), |c| {
        c.name.as_deref().unwrap_or("(restricted)")
    });

    app.tabs.render(frame, layout[1]);

    let inside = layout[1].inner(Margin::new(1, 1));
    let below_tabs = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(inside);

    if let Some(tab) = app.tabs.selected() {
        match tab.kind() {
            TabKind::Modules => {
                app.modules.render(frame, below_tabs[1], Block::new(), |m| {
                    m.name.as_deref().unwrap_or("(unnamed)")
                });
            }
            _ => {}
        }
    }
}
