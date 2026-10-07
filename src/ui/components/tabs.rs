use crate::api::models::Tabs;
use crate::ui::components::pane::pane;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{self, Block},
};

pub fn nav_links<'a, T>(tabs: Vec<T>, pane: Block<'a>) -> widgets::Tabs<'a>
where
    T: Into<Line<'a>>,
{
    widgets::Tabs::new(tabs)
        .block(pane)
        .style(Color::White)
        .highlight_style(Style::default().cyan().bold())
        .divider(symbols::DOT)
        .padding(" ", " ")
}

const PINNED: [&str; 4] = ["modules", "assignments", "announcements", "grades"];

fn tab_sort_key(tab: &Tabs) -> (usize, i32) {
    let rank = tab
        .id
        .as_deref()
        .and_then(|id| PINNED.iter().position(|p| *p == id))
        .unwrap_or(PINNED.len());
    (rank, tab.position)
}

pub struct TabsState {
    pub tabs: Vec<Tabs>,
    pub selected: usize,
    pub offset: usize,
}

impl TabsState {
    pub fn new(mut tabs: Vec<Tabs>) -> Self {
        tabs.sort_by_key(tab_sort_key);
        Self {
            tabs,
            selected: 0,
            offset: 0,
        }
    }

    pub fn selected(&self) -> Option<&Tabs> {
        self.tabs.get(self.selected)
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        let len = self.tabs.len();
        if len == 0 {
            return false;
        }
        let i = self.selected;
        let next = match key.code {
            KeyCode::Char('[') | KeyCode::Char('h') | KeyCode::Left => {
                (self.selected + len - 1) % len
            }
            KeyCode::Char(']') | KeyCode::Char('l') | KeyCode::Right => (self.selected + 1) % len,
            _ => return false,
        };
        self.selected = next;
        next != i
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<Line> = self
            .tabs
            .iter()
            .map(|c| Line::from(c.label.as_deref().unwrap_or("(restricted)")))
            .collect();
        if items.is_empty() {
            return;
        }

        let budget = area.width.saturating_sub(2) as usize;
        let widths: Vec<usize> = items.iter().map(|l| l.width() + 3).collect();
        let fits = |from: usize, to: usize| widths[from..=to].iter().sum::<usize>() <= budget;

        if self.selected < self.offset {
            self.offset = self.selected;
        }

        while self.offset < self.selected && !fits(self.offset, self.selected) {
            self.offset += 1;
        }

        let mut end = self.selected + 1;
        while end < items.len() && fits(self.offset, end) {
            end += 1;
        }

        let left = if self.offset > 0 { "◀" } else { "" };
        let right = if end < items.len() { "▶" } else { "" };
        let pane = pane("")
            .title_top(Line::from(left).left_aligned())
            .title_top(Line::from(right).right_aligned());

        let visible: Vec<Line> = items[self.offset..end].to_vec();
        let tabs = nav_links(visible, pane).select(self.selected - self.offset);
        frame.render_widget(tabs, area);
    }
}
