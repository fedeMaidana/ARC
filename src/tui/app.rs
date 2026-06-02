// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;
use ratatui::DefaultTerminal;

use super::{event, render};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run() -> Result<()> {
    ratatui::run(run_loop)?;

    Ok(())
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug)]
pub struct TuiApp {
    current_screen: Screen,
    should_quit: bool,
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Audit,
    Decision,
    Config,
}

// ─── < Implementations > ────────────────────────────────────────────

impl TuiApp {
    fn new() -> Self {
        Self {
            current_screen: Screen::Audit,
            should_quit: false,
        }
    }

    pub fn current_screen(&self) -> Screen {
        self.current_screen
    }

    pub fn show_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn should_quit(&self) -> bool {
        self.should_quit
    }
}

impl Screen {
    pub const ALL: [Self; 3] = [Self::Audit, Self::Decision, Self::Config];

    pub fn key(self) -> &'static str {
        match self {
            Self::Audit => "1",
            Self::Decision => "2",
            Self::Config => "3",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Audit => "Audit log",
            Self::Decision => "Decision playground",
            Self::Config => "Config summary",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Audit => "Inspect recent ARC activity and understand what was allowed, denied, or marked as ask.",
            Self::Decision => "Simulate a request and see how ARC would classify it before anything runs.",
            Self::Config => "Review the active ARC config path, policy shape, console rules, and HTTP protections.",
        }
    }

    pub fn status(self) -> &'static str {
        match self {
            Self::Audit => "Foundation ready. Audit log reading comes next.",
            Self::Decision => "Foundation ready. Interactive request input comes next.",
            Self::Config => "Foundation ready. Loaded config rendering comes next.",
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn run_loop(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = TuiApp::new();

    while !app.should_quit() {
        terminal.draw(|frame| render::render(frame, &app))?;

        event::handle_next_event(&mut app)?;
    }

    Ok(())
}
