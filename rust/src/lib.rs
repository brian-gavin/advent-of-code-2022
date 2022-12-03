use std::io;

pub mod one;
pub mod two;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
