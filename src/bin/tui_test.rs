#[path = "../tui.rs"]
mod tui;

use crossterm::event;

#[path = "../monitor.rs"]
mod monitor;

use std::time::Duration;
use sysinfo::System;

fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        let mut sys = System::new();
        loop {
            let stats = monitor::get_system_stats(&mut sys);
            terminal.draw(|frame| tui::render(frame, &stats))?;

            if event::poll(Duration::from_secs(1))? && event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
