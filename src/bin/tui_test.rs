#[path = "../tui.rs"]
mod tui;

use crossterm::event;

fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(tui::render)?;

            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
