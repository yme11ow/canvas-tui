use crate::api::models::{Course, Tabs};
use crate::ui::components::tabs::nav_links;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
// use crate::ui::components::list::create_list;
use crate::ui::components::pane::pane;
use crate::ui::layout::build_layout;

pub struct CoursesState {
    pub courses: Vec<Course>,
    pub list: ListState,
}

impl CoursesState {
    pub fn new(courses: Vec<Course>) -> Self {
        let mut list = ListState::default();
        if !courses.is_empty() {
            list.select(Some(0));
        }
        Self { courses, list }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let len = self.courses.len();
        if len == 0 {
            return;
        }
        let i = self.list.selected().unwrap_or(0);
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.list.select(Some((i + 1) % len)),
            KeyCode::Char('k') | KeyCode::Up => self.list.select(Some((i + len - 1) % len)),
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let layout = build_layout(frame);
        let left_pane = pane("courses");

        /*
        let course_list = create_list(courses, |c| {
            c.name.clone().unwrap_or_else(|| "(restricted)".to_string())
        });
        */

        let items: Vec<ListItem> = self
            .courses
            .iter()
            .map(|c| ListItem::new(c.name.as_deref().unwrap_or("(restricted)")))
            .collect();

        // frame.render_widget(List::new(items).block(left_pane), layout[0]);
        // frame.render_widget(right_pane, layout[1]);
        let course_list = List::new(items)
            .block(left_pane)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(course_list, layout[0], &mut self.list);
    }
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
    pub offset: usize
}

impl TabsState {
    pub fn new(mut tabs: Vec<Tabs>) -> Self {
        tabs.sort_by_key(tab_sort_key);
        Self { tabs, selected: 0, offset: 0 }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let len = self.tabs.len();
        if len == 0 { return; }
        match key.code {
            KeyCode::Char('[') | KeyCode::Char('h') => self.selected = (self.selected + len - 1) % len,
            KeyCode::Char(']') | KeyCode::Char('l') => self.selected = (self.selected + 1) % len,
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let layout = build_layout(frame);
        // let right_pane = pane("");
        let area = layout[1];

        let items: Vec<Line> = self
            .tabs
            .iter()
            .map(|c| Line::from(c.label.as_deref().unwrap_or("(restricted)")))
            .collect();
        if items.is_empty() { return; }

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
