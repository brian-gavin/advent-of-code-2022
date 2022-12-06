use std::io;

pub mod five;
pub mod four;
pub mod one;
pub mod three;
pub mod two;
pub mod six;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
