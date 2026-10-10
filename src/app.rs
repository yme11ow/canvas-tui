use crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;

use crate::{
    api::{
        client::CanvasClient, models::{Course, Module, TabKind, Tabs},
    }, ui::components::{list::SelectList, tabs::TabsState},
};

pub struct App {
    pub client: CanvasClient,
    pub courses: SelectList<Course>,
    pub tabs: TabsState,
    pub modules: SelectList<Module>,
    tab_cache: HashMap<u64, Vec<Tabs>>,
    module_cache: HashMap<u64, Vec<Module>>,
    pub should_quit: bool,
}

impl App {
    pub fn new(client: CanvasClient) -> reqwest::Result<Self> {
        let courses = client.courses()?;
        Ok(Self {
            client,
            courses: SelectList::new(courses),
            tabs: TabsState::new(Vec::new()),
            modules: SelectList::new(Vec::new()),
            tab_cache: HashMap::new(),
            module_cache: HashMap::new(),
            should_quit: false,
        })
    }

    fn load_tabs(&mut self) {
        let Some(course) = self.courses.selected() else {
            return;
        };
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

    fn load_content(&mut self) {
        let Some(course) = self.courses.selected() else {
            return;
        };
        let course_id = course.id;
        let Some(tab) = self.tabs.selected() else {
            return;
        };
        match tab.kind() {
            TabKind::Modules => {
                if !self.module_cache.contains_key(&course_id) {
                    if let Ok(modules) = self.client.modules(course_id) {
                        self.module_cache.insert(course_id, modules);
                    }
                }
                if let Some(modules) = self.module_cache.get(&course_id) {
                    self.modules = SelectList::new(modules.clone());
                }
            }
            _ => {}
        }
    }

    fn refresh(&mut self) {
        self.tab_cache.clear();
        self.module_cache.clear();
        self.load_tabs();
        self.load_content();
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('r') => self.refresh(),
            KeyCode::Char('[')
            | KeyCode::Char('h')
            | KeyCode::Char(']')
            | KeyCode::Char('l')
            | KeyCode::Left
            | KeyCode::Right => {
                if self.tabs.handle_key(key) {
                    self.load_content();
                }
            }
            // left pane
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_courses(true);
            }
            KeyCode::Char('k') |KeyCode::Up => {
                self.move_courses(false);
            }

            // right pane
            KeyCode::Char('d') => {
                self.move_content(true);
            }
            KeyCode::Char('u') => {
                self.move_content(false);
            }
            _ => {}
        }
    }

    pub fn move_courses(&mut self, down: bool) {
        let changed = if down { self.courses.next() } else { self.courses.prev() };
        if changed {
            self.load_tabs();
            self.load_content();
        }
    }

    pub fn move_content(&mut self, down: bool) {
        match self.tabs.selected().map(|t| t.kind()) {
            Some(TabKind::Modules) => {
                if down { self.modules.next(); } else { self.modules.prev(); }
            }
            _ => {}
        }
    }
}
