mod ui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    ratatui::run(|terminal| loop {
        terminal.draw(ui::views::courses::render)?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    })
}
