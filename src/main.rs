// ─── < Entry Point > ────────────────────────────────────────────────

fn main() {
    let exit_code = arc::app::run();

    std::process::exit(exit_code);
}
