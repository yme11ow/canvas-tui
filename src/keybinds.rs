use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Quit,
    Refresh,
    NextTab,
    PrevTab,
    CourseDown,
    CourseUp,
    ContentDown,
    ContentUp,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Action::Quit => "quit",
            Action::Refresh => "refresh",
            Action::NextTab => "next_tab",
            Action::PrevTab => "prev_tab",
            Action::CourseDown => "course_down",
            Action::CourseUp => "course_up",
            Action::ContentDown => "content_down",
            Action::ContentUp => "content_up",
        };
        f.write_str(name)
    }
}

pub const DEFAULTS: &[(Action, &[&str])] = &[
    (Action::Quit, &["q"]),
    (Action::Refresh, &["r"]),
    (Action::NextTab, &["]", "l", "right"]),
    (Action::PrevTab, &["[", "h", "left"]),
    (Action::CourseDown, &["j", "down"]),
    (Action::CourseUp, &["k", "up"]),
    (Action::ContentDown, &["d"]),
    (Action::ContentUp, &["u"]),
];

/// A key press with modifiers, ignoring event kind/state so lookups match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    code: KeyCode,
    mods: KeyModifiers,
}

impl Key {
    fn new(code: KeyCode, mut mods: KeyModifiers) -> Self {
        // The char already says whether shift was held ('G' vs 'g').
        if let KeyCode::Char(_) = code {
            mods.remove(KeyModifiers::SHIFT);
        }
        Self { code, mods }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Key::new(event.code, event.modifiers)
    }
}

/// Parses keys like `j`, `G`, `down`, `ctrl-d`, `alt-shift-tab`.
pub fn parse_key(s: &str) -> Result<Key, String> {
    let (mod_part, key_part) = match s.rsplit_once('-') {
        // `-` on its own, or a trailing `-` as in `ctrl--`
        Some((mods, "")) => (mods.strip_suffix('-').unwrap_or(mods), "-"),
        Some((mods, key)) => (mods, key),
        None => ("", s),
    };

    let mut mods = KeyModifiers::NONE;
    for m in mod_part.split('-').filter(|m| !m.is_empty()) {
        mods |= match m.to_ascii_lowercase().as_str() {
            "ctrl" => KeyModifiers::CONTROL,
            "alt" => KeyModifiers::ALT,
            "shift" => KeyModifiers::SHIFT,
            _ => return Err(format!("unknown modifier `{m}` in `{s}`")),
        };
    }

    let mut chars = key_part.chars();
    let code = match (chars.next(), chars.next()) {
        (Some(c), None) => KeyCode::Char(c),
        _ => match key_part.to_ascii_lowercase().as_str() {
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "enter" => KeyCode::Enter,
            "esc" => KeyCode::Esc,
            "tab" if mods.contains(KeyModifiers::SHIFT) => KeyCode::BackTab,
            "tab" => KeyCode::Tab,
            "backspace" => KeyCode::Backspace,
            "delete" => KeyCode::Delete,
            "space" => KeyCode::Char(' '),
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            f if f.starts_with('f') && f[1..].parse::<u8>().is_ok() => {
                KeyCode::F(f[1..].parse().unwrap())
            }
            _ => return Err(format!("unknown key `{key_part}` in `{s}`")),
        },
    };
    Ok(Key::new(code, mods))
}

pub struct Keymap(HashMap<Key, Action>);

impl Keymap {
    /// Builds the default keymap, then replaces the keys of every action in `overrides`.
    pub fn new(overrides: &HashMap<Action, Vec<String>>) -> Result<Self, String> {
        let mut map = HashMap::new();
        for (action, keys) in DEFAULTS {
            if overrides.contains_key(action) {
                continue;
            }
            for k in *keys {
                map.insert(parse_key(k).expect("default keybind should parse"), *action);
            }
        }

        // User binds win over defaults, but two user binds can't share a key.
        let mut user: HashMap<Key, Action> = HashMap::new();
        for (action, keys) in overrides {
            for k in keys {
                if let Some(other) = user.insert(parse_key(k)?, *action)
                    && other != *action
                {
                    return Err(format!("`{k}` is bound to both `{other}` and `{action}`"));
                }
            }
        }
        map.extend(user);

        Ok(Self(map))
    }

    pub fn get(&self, event: KeyEvent) -> Option<Action> {
        self.0.get(&Key::from(event)).copied()
    }
}
