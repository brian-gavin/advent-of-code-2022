use std::io;

pub mod one;

pub fn read_input() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}
