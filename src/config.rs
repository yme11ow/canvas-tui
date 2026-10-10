use crate::keybinds::{Action, DEFAULTS};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub keybinds: HashMap<Action, Vec<String>>,
}

pub fn path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("canvas-tui").join("config.toml"))
}

/// Loads the config file, or the defaults if it doesn't exist.
///
/// In release builds, a missing file is replaced with a commented-out starter config.
pub fn load() -> Result<Config, String> {
    let Some(path) = path() else {
        return Ok(Config::default());
    };
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if !cfg!(debug_assertions) {
                // Best effort: failing to write the starter shouldn't stop the app.
                let _ = fs::create_dir_all(path.parent().unwrap())
                    .and_then(|_| fs::write(&path, starter()));
            }
            return Ok(Config::default());
        }
        Err(e) => return Err(format!("couldn't read {}: {e}", path.display())),
    };
    toml::from_str(&text).map_err(|e| format!("invalid config at {}:\n{e}", path.display()))
}

/// A config file with every setting commented out, generated from the defaults.
fn starter() -> String {
    let mut out = String::from(
        "# canvas-tui config\n\
         # Uncomment a line to change it. Each action you set replaces its default keys.\n\
         # Keys: a character (j, G, [), or up, down, left, right, enter, esc, tab,\n\
         # backspace, delete, space, home, end, pageup, pagedown, f1-f12.\n\
         # Modifiers: ctrl-, alt-, shift- (e.g. \"ctrl-d\", \"shift-tab\").\n\
         \n\
         [keybinds]\n",
    );
    for (action, keys) in DEFAULTS {
        let keys: Vec<String> = keys.iter().map(|k| format!("{k:?}")).collect();
        out += &format!("# {action} = [{}]\n", keys.join(", "));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_is_inert() {
        let config: Config = toml::from_str(&starter()).unwrap();
        assert!(config.keybinds.is_empty());
    }

    #[test]
    fn uncommented_starter_matches_defaults() {
        let uncommented = starter().replace("\n# ", "\n");
        let uncommented = uncommented.split_once("[keybinds]").unwrap().1;
        let config: Config = toml::from_str(&format!("[keybinds]{uncommented}")).unwrap();
        for (action, keys) in DEFAULTS {
            assert_eq!(config.keybinds[action], *keys);
        }
    }
}
