use std::io;

pub mod five;
pub mod four;
pub mod one;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
