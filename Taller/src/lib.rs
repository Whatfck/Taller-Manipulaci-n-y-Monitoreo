use std::io::{self, Write};

// pub use taller::clear;
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = io::stdout().flush();
}