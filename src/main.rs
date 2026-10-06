mod api;
mod app;
mod ui;
use crossterm::event::{self, Event, KeyEventKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = api::client::CanvasClient::from_env()?;
    let mut app = app::App::new(client)?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                app.courses.render(frame);
                app.tabs.render(frame);
            })?;

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
