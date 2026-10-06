use crate::api::models::{Course, Tabs};
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
        if len == 0 { return; }
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

pub struct TabsState {
    pub tabs: Vec<Tabs>,
    pub list: ListState,
}

impl TabsState {
    pub fn new(tabs: Vec<Tabs>) -> Self {
        let mut list = ListState::default();
        if !tabs.is_empty() {
            list.select(Some(0));
        }

        Self { tabs, list }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let layout = build_layout(frame);
        let right_pane = pane("");

        let items: Vec<ListItem> = self
            .tabs
            .iter()
            .map(|c| ListItem::new(c.label.as_deref().unwrap_or("(restricted)")))
            .collect();

        let tabs = List::new(items)
            .block(right_pane)
            .highlight_symbol(">")
            .highlight_style(Style::default().fg(Color::Cyan));
        
        frame.render_stateful_widget(tabs, layout[1], &mut self.list);

    }
}
