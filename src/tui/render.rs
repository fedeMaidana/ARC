// ─── < Imports > ────────────────────────────────────────────────────

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::app::{Screen, TuiApp};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn render(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(8), Constraint::Length(3)])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_body(frame, chunks[1], app);
    render_footer(frame, chunks[2]);
}

// ─── < Private Functions > ──────────────────────────────────────────

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("ARC", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" · Action Review Controller"),
        ]),
        Line::from("A controlled passage for commands, agents, scripts, and automation."),
        Line::from("Use 1/2/3 to switch screens. Press q or Esc to quit."),
    ])
    .block(Block::default().title(" ARC TUI ").borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let screen = app.current_screen();

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(28), Constraint::Min(20)])
        .split(area);

    render_navigation(frame, body_chunks[0], screen);
    render_screen(frame, body_chunks[1], screen);
}

fn render_navigation(frame: &mut Frame, area: Rect, selected_screen: Screen) {
    let mut lines = Vec::new();

    for screen in Screen::ALL {
        let marker = if screen == selected_screen { ">" } else { " " };

        lines.push(Line::from(format!("{marker} {}. {}", screen.key(), screen.title())));
    }

    let navigation = Paragraph::new(lines).block(Block::default().title(" Screens ").borders(Borders::ALL));

    frame.render_widget(navigation, area);
}

fn render_screen(frame: &mut Frame, area: Rect, screen: Screen) {
    let screen_content = Paragraph::new(vec![
        Line::from(Span::styled(screen.title(), Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(screen.description()),
        Line::from(""),
        Line::from(screen.status()),
        Line::from(""),
        Line::from("This first TUI slice is intentionally read-only."),
        Line::from("Next step: connect the Audit log screen to ARC audit JSONL events."),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().title(" Current screen ").borders(Borders::ALL));

    frame.render_widget(screen_content, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("q/Esc quit · 1 Audit · 2 Decision · 3 Config").block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
