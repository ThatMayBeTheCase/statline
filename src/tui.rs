use crate::monitor::SystemStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn render(frame: &mut Frame, stats: &SystemStats) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(frame.area());

    let system_panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(rows[0]);

    let used_gib = stats.used_ram as f64 / 1024.0_f64.powi(3);
    let total_gib = stats.total_ram as f64 / 1024.0_f64.powi(3);

    let ram_percent = if stats.total_ram > 0 {
        (stats.used_ram as f64 / stats.total_ram as f64 * 100.0) as u16
    } else {
        0
    };

    let cpu = Paragraph::new(format!("Usage: {:.1}%", stats.cpu_usage))
        .block(Block::default().title(" CPU ").borders(Borders::ALL));

    let gpu =
        Paragraph::new("Usage: 47%").block(Block::default().title(" GPU ").borders(Borders::ALL));

    let ram = Gauge::default()
        .block(Block::default().title(" RAM ").borders(Borders::ALL))
        .percent(ram_percent.min(100))
        .label(format!("{:.1} / {:.1} GiB", used_gib, total_gib));

    let active_sessions = Paragraph::new("No active sessions").block(
        Block::default()
            .title(" Active Sessions ")
            .borders(Borders::ALL),
    );

    let game_library = Paragraph::new("No games loaded").block(
        Block::default()
            .title(" Game Library ")
            .borders(Borders::ALL),
    );

    frame.render_widget(cpu, system_panels[0]);
    frame.render_widget(gpu, system_panels[1]);
    frame.render_widget(ram, system_panels[2]);

    frame.render_widget(active_sessions, rows[1]);

    frame.render_widget(game_library, rows[2]);
}
