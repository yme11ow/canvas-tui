use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};

pub struct SelectList<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T> SelectList<T> {
    pub fn new(items: Vec<T>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self { items, state }
    }

    pub fn selected(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    pub fn step(&mut self, delta: isize) -> bool {
        let len = self.items.len();
        if len == 0 { return false; }
        let i = self.state.selected().unwrap_or(0);
        let next = (i as isize + delta).rem_euclid(len as isize) as usize;
        self.state.select(Some(next));
        next != i
    }

    pub fn next(&mut self) -> bool {
        self.step(1)
    }

    pub fn prev(&mut self) -> bool {
        self.step(-1)
    }

    pub fn render<'a>(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        block: Block<'a>,
        label: impl Fn(&T) -> &str,
    ) {
        let items: Vec<ListItem> = self.items.iter().map(|t| ListItem::new(label(t))).collect();
        let list = List::new(items)
            .block(block)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
