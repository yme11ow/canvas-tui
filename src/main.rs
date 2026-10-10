mod api;
mod app;
mod config;
mod keybinds;
mod ui;
use crossterm::event::{self, Event, KeyEventKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let keymap = config::load()
        .and_then(|config| {
            keybinds::Keymap::new(&config.keybinds).map_err(|e| {
                let path = config::path().unwrap_or_default();
                format!("invalid keybind in {}: {e}", path.display())
            })
        })
        .unwrap_or_else(|e| {
            eprintln!("canvas-tui: {e}");
            std::process::exit(1);
        });
    let client = api::client::CanvasClient::from_env()?;
    let mut app = app::App::new(client, keymap)?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| ui::views::courses::render(&mut app, frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
            if app.should_quit {
                break Ok(());
            }
        }
    })
}
