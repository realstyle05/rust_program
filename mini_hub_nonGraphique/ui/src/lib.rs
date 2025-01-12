pub fn clear_terminal() {
    let term = console::Term::stdout();
    term.clear_screen().expect("failed clearing screen");
}