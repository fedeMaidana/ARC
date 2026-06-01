// ─── < Imports > ────────────────────────────────────────────────────

use std::io::{self, Stdout, Write};

use crossterm::cursor::{MoveToColumn, MoveToNextLine, RestorePosition, SavePosition};
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{ExecutableCommand, queue};
use thiserror::Error;

use crate::ui;

// ─── < Constants > ──────────────────────────────────────────────────

const OPTION_WIDTH: usize = 18;
const CONTENT_SPACING: &str = "  ";

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AskError {
    #[error("failed to prepare terminal ask prompt")]
    PrepareTerminal {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to render terminal ask prompt")]
    Render {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to read terminal ask input")]
    ReadInput {
        #[source]
        source: std::io::Error,
    },
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AskAnswer {
    Yes,
    No,
}

// ─── < Structs > ────────────────────────────────────────────────────

struct RawModeGuard;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn ask_yes_no(prompt: &str) -> Result<AskAnswer, AskError> {
    println!();
    print_prompt_header(prompt);

    let _raw_mode_guard = RawModeGuard::new().map_err(|source| AskError::PrepareTerminal { source })?;

    let mut stdout = io::stdout();

    stdout.execute(SavePosition).map_err(|source| AskError::Render { source })?;

    let mut selected_answer = AskAnswer::No;

    render_selector(&mut stdout, selected_answer).map_err(|source| AskError::Render { source })?;

    loop {
        let event = event::read().map_err(|source| AskError::ReadInput { source })?;

        let Event::Key(key_event) = event else {
            continue;
        };

        match key_event.code {
            KeyCode::Up | KeyCode::Left | KeyCode::Char('k') => {
                selected_answer = AskAnswer::Yes;
                render_selector(&mut stdout, selected_answer).map_err(|source| AskError::Render { source })?;
            }
            KeyCode::Down | KeyCode::Right | KeyCode::Char('j') | KeyCode::Tab => {
                selected_answer = AskAnswer::No;
                render_selector(&mut stdout, selected_answer).map_err(|source| AskError::Render { source })?;
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                finish_selector(&mut stdout, AskAnswer::Yes).map_err(|source| AskError::Render { source })?;

                return Ok(AskAnswer::Yes);
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                finish_selector(&mut stdout, AskAnswer::No).map_err(|source| AskError::Render { source })?;

                return Ok(AskAnswer::No);
            }
            KeyCode::Enter => {
                finish_selector(&mut stdout, selected_answer).map_err(|source| AskError::Render { source })?;

                return Ok(selected_answer);
            }
            _ => {}
        }
    }
}

// ─── < Implementations > ────────────────────────────────────────────

impl RawModeGuard {
    fn new() -> Result<Self, std::io::Error> {
        terminal::enable_raw_mode()?;

        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn print_prompt_header(prompt: &str) {
    println!("{}", guide());
    println!("{}{}{}", marker("◇"), CONTENT_SPACING, ui::bold("Approval required:"));
    println!("{}{}{}", guide(), CONTENT_SPACING, prompt);
    println!("{}", guide());
    println!("{}{}{}", marker("◆"), CONTENT_SPACING, ui::bold("Select an option:"));
}

fn render_selector(stdout: &mut Stdout, selected_answer: AskAnswer) -> Result<(), std::io::Error> {
    queue!(stdout, RestorePosition, Clear(ClearType::FromCursorDown))?;

    render_option(stdout, "Yes", selected_answer == AskAnswer::Yes)?;
    queue!(stdout, MoveToNextLine(1), MoveToColumn(0))?;

    render_option(stdout, "No", selected_answer == AskAnswer::No)?;
    queue!(stdout, MoveToNextLine(1), MoveToColumn(0))?;

    render_footer(stdout)?;

    stdout.flush()
}

fn finish_selector(stdout: &mut Stdout, answer: AskAnswer) -> Result<(), std::io::Error> {
    render_selector(stdout, answer)?;
    queue!(stdout, MoveToNextLine(1), MoveToColumn(0))?;
    stdout.flush()
}

fn render_option(stdout: &mut Stdout, label: &str, is_selected: bool) -> Result<(), std::io::Error> {
    let circle = if is_selected { "●" } else { "○" };
    let content = format!("{CONTENT_SPACING}{circle} {label}");
    let padded_content = pad_right(&content, OPTION_WIDTH);

    queue!(stdout, Print(guide()))?;

    if is_selected {
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Bold),
            Print(padded_content),
            ResetColor,
            SetAttribute(Attribute::Reset)
        )?;
    } else {
        queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(padded_content), ResetColor)?;
    }

    Ok(())
}

fn render_footer(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    queue!(stdout, Print(footer()))
}

fn marker(symbol: &str) -> String {
    ui::cyan(symbol)
}

fn guide() -> String {
    ui::cyan("│")
}

fn footer() -> String {
    ui::cyan("└")
}

fn pad_right(value: &str, width: usize) -> String {
    let current_width = value.chars().count();

    if current_width >= width {
        return value.to_string();
    }

    let padding = " ".repeat(width - current_width);

    format!("{value}{padding}")
}
