use crossterm::event::{KeyCode, KeyEvent};

use crate::{api::models::Course, ui::views::courses::CoursesState};

pub struct App {
    pub courses: CoursesState,
    pub should_quit: bool,
}

impl App {
    pub fn new(courses: Vec<Course>) -> Self {
        Self { courses: CoursesState::new(courses), should_quit: false }
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            _ => CoursesState::handle_key(&mut self.courses, key),
        }
    }
}