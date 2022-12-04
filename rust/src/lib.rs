use std::io;

pub mod four;
pub mod one;
pub mod three;
pub mod two;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
