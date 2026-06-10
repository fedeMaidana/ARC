// ─── < ANSI Styles > ────────────────────────────────────────────────

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

// ─── < Public Functions > ───────────────────────────────────────────

pub fn bold(text: &str) -> String {
    format!("{BOLD}{text}{RESET}")
}

pub fn dim(text: &str) -> String {
    format!("{DIM}{text}{RESET}")
}

pub fn red(text: &str) -> String {
    format!("{RED}{text}{RESET}")
}

pub fn green(text: &str) -> String {
    format!("{GREEN}{text}{RESET}")
}

pub fn yellow(text: &str) -> String {
    format!("{YELLOW}{text}{RESET}")
}

pub fn cyan(text: &str) -> String {
    format!("{CYAN}{text}{RESET}")
}

pub fn section(title: &str) -> String {
    cyan(&format!("▶ {title}"))
}

pub fn indent_lines(text: &str, spaces: usize) -> String {
    let padding = " ".repeat(spaces);

    text.trim_end()
        .lines()
        .map(|line| format!("{padding}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}
