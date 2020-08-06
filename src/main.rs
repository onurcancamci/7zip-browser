mod term;

pub use term::*;

fn main() {
    let mut term = Term::new();
    term.ui_loop();
}
