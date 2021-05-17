//! Utility functions.

use std::str::FromStr;

/// Reads a line from stdin.
/// Panics on error.
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

/// Reads and parses a f64 from stdin.
/// Panics on error and invalid number.
pub fn read_f64() -> f64 {
    f64::from_str(read_line().trim()).expect("Not a Number.")
}

/// Credit: <https://internals.rust-lang.org/t/create-a-flushing-version-of-print/9870/6>
#[macro_export]
macro_rules! print_flush {
    ( $($t:tt)* ) => {
        {
            let mut h = std::io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}
