use crossterm::event::KeyEvent;
use std::collections::HashMap;

use crate::{
    api::{
        client::CanvasClient, models::{Course, Module, TabKind, Tabs},
    },
    keybinds::{Action, Keymap},
    ui::components::{list::SelectList, tabs::TabsState},
};

pub struct App {
    pub client: CanvasClient,
    keymap: Keymap,
    pub courses: SelectList<Course>,
    pub tabs: TabsState,
    pub modules: SelectList<Module>,
    tab_cache: HashMap<u64, Vec<Tabs>>,
    module_cache: HashMap<u64, Vec<Module>>,
    pub should_quit: bool,
}

impl App {
    pub fn new(client: CanvasClient, keymap: Keymap) -> reqwest::Result<Self> {
        let courses = client.courses()?;
        Ok(Self {
            client,
            keymap,
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
        let Some(action) = self.keymap.get(key) else {
            return;
        };
        match action {
            Action::Quit => self.should_quit = true,
            Action::Refresh => self.refresh(),
            Action::NextTab => self.move_tabs(true),
            Action::PrevTab => self.move_tabs(false),
            // left pane
            Action::CourseDown => self.move_courses(true),
            Action::CourseUp => self.move_courses(false),
            // right pane
            Action::ContentDown => self.move_content(true),
            Action::ContentUp => self.move_content(false),
        }
    }

    pub fn move_tabs(&mut self, right: bool) {
        let changed = if right { self.tabs.next() } else { self.tabs.prev() };
        if changed {
            self.load_content();
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
