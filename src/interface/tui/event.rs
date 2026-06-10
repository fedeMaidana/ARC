// ─── < Imports > ────────────────────────────────────────────────────

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::app::{Screen, TuiApp};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn handle_next_event(app: &mut TuiApp) -> io::Result<()> {
    let Event::Key(key_event) = event::read()? else {
        return Ok(());
    };

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('1') => app.show_screen(Screen::Audit),
        KeyCode::Char('2') => app.show_screen(Screen::Decision),
        KeyCode::Char('3') => app.show_screen(Screen::Config),
        _ => {}
    }

    Ok(())
}
