use std::{env::args, fmt::Display, process};

use aoc::*;

fn run(problem: &str, input: Vec<String>) -> Result<Box<dyn Display>, String> {
    Ok(match problem {
        "11" => Box::new(one::solve1(input)),
        "12" => Box::new(one::solve2(input)),
        "21" => Box::new(two::solve1(input)),
        "22" => Box::new(two::solve2(input)),
        "31" => Box::new(three::solve1(input)),
        "32" => Box::new(three::solve2(input)),
        "41" => Box::new(four::solve1(input)),
        "42" => Box::new(four::solve2(input)),
        "51" => Box::new(five::solve1(input)),
        "52" => Box::new(five::solve2(input)),
        _ => return Err(format!("invalid problem number: {}", problem)),
    })
}

fn main() {
    let args: Vec<_> = args().collect();
    let Some(problem) = args.get(1) else {
        eprintln!("expected a problem number");
        process::exit(1);
    };
    let input = aoc::read_input().unwrap();
    match run(problem, input) {
        Ok(solution) => println!("{}", solution),
        Err(e) => eprintln!("error: {}", e),
    }
}
