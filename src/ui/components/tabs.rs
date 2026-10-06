use ratatui::{
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Tabs},
};

pub fn nav_links<'a, T>(tabs: Vec<T>, pane: Block<'a>) -> Tabs<'a>
where
    T: Into<Line<'a>>,
{
    Tabs::new(tabs)
        .block(pane)
        .style(Color::White)
        .highlight_style(Style::default().cyan().bold())
        .divider(symbols::DOT)
        .padding(" ", " ")
}