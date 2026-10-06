use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use crate::api::models::Course;
// use crate::ui::components::list::create_list;
use crate::ui::layout::build_layout;
use crate::ui::components::pane::pane;

pub struct CoursesState {
    pub courses: Vec<Course>,
    pub list: ListState,
}

impl CoursesState {
    pub fn new(courses: Vec<Course>) -> Self {
        let mut list = ListState::default();
        if !courses.is_empty() {list.select(Some(0)); }
        Self { courses, list }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.list.select_previous(),
            _=> {}
        }
    }


    pub fn render(&mut self, frame: &mut Frame) {
        let layout = build_layout(frame);
        let left_pane = pane("courses");
        let right_pane = pane("");

        /*
        let course_list = create_list(courses, |c| {
            c.name.clone().unwrap_or_else(|| "(restricted)".to_string())
        });
        */

        let items: Vec<ListItem> = self.courses
            .iter()
            .map(|c| ListItem::new(c.name.as_deref().unwrap_or("(restricted)")))
            .collect();

        // frame.render_widget(List::new(items).block(left_pane), layout[0]);
        // frame.render_widget(right_pane, layout[1]);
        let course_list = List::new(items).block(left_pane).highlight_symbol("> ");
        frame.render_stateful_widget(course_list, layout[0], &mut self.list);
        frame.render_widget(right_pane, layout[1]);
    }
}

/* 
pub fn render(frame: &mut Frame, courses: &[Course]) {
    let layout = build_layout(frame);
    let left_pane = pane("courses");
    let right_pane = pane("");

    /*
    let course_list = create_list(courses, |c| {
        c.name.clone().unwrap_or_else(|| "(restricted)".to_string())
    });
    */

    let items: Vec<ListItem> = courses
        .iter()
        .map(|c| ListItem::new(c.name.as_deref().unwrap_or("(restricted)")))
        .collect();

    frame.render_widget(List::new(items).block(left_pane), layout[0]);
    frame.render_widget(right_pane, layout[1]);
}
*/