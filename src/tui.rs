use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame) {
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

    let cpu =
        Paragraph::new("Usage: 24%").block(Block::default().title(" CPU ").borders(Borders::ALL));

    let gpu =
        Paragraph::new("Usage: 47%").block(Block::default().title(" GPU ").borders(Borders::ALL));

    let ram =
        Paragraph::new("Usage: 70%").block(Block::default().title(" RAM ").borders(Borders::ALL));

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
