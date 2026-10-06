mod ui;
mod api;
mod app;
use crossterm::event::{self, Event, KeyEventKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();                                
    let client = api::client::CanvasClient::from_env()?;  
    let mut app = app::App::new(client.courses()?);
    ratatui::run(|terminal| loop {
        terminal.draw(|frame| app.courses.render(frame))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press { app.handle_key(key); }
        }
        if app.should_quit { break Ok(()); }
    })
}


/*
let mut app = app::App::new(client.courses()?);
ratatui::run(|terminal| loop {
    terminal.draw(|frame| ui::views::courses::render(frame, &mut app.courses))?;
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press { app.handle_key(key); }
    }
    if app.should_quit { break Ok(()); }
})

*/