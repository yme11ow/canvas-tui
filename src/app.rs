use crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;

use crate::{
    api::{client::CanvasClient, models::Tabs},
    ui::views::courses::{CoursesState, TabsState},
};


pub struct App {
    pub client: CanvasClient,
    pub courses: CoursesState,
    pub tabs: TabsState,
    tab_cache: HashMap<u64, Vec<Tabs>>,
    pub should_quit: bool,
}

impl App {
    pub fn new(client: CanvasClient) -> reqwest::Result<Self> {
        let courses = client.courses()?;
        Ok(Self {
            client,
            courses: CoursesState::new(courses),
            tabs: TabsState::new(Vec::new()),
            tab_cache: HashMap::new(),
            should_quit: false,
        })
    }

    fn load_tabs(&mut self) {
        let Some(course) = self.courses.list.selected().and_then(|i| self.courses.courses.get(i)) else { return };
        let course_id = course.id;

        if !self.tab_cache.contains_key(&course_id) {
            if let Ok(tabs) = self.client.tabs(course_id) {
                self.tab_cache.insert(course_id, tabs);
            }
        }
        if let Some(tabs) = self.tab_cache.get(&course_id) {
            self.tabs = TabsState::new(tabs.clone());
        }
    }

    fn refresh(&mut self) {
        self.tab_cache.clear();
        self.load_tabs();
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('r') => self.refresh(),
            _ => {
                CoursesState::handle_key(&mut self.courses, key);
                self.load_tabs();
            }
        }
    }
}
