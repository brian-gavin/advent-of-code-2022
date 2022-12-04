use std::{env::args, process};

use aoc::*;

fn run(problem: &str, input: Vec<String>) -> Result<impl ToString, String> {
    Ok(match problem {
        "11" => one::solve1(input),
        "12" => one::solve2(input),
        "21" => two::solve1(input),
        "22" => two::solve2(input),
        "31" => three::solve1(input),
        "32" => three::solve2(input),
        "41" => four::solve1(input),
        "42" => four::solve2(input),
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
        Ok(solution) => println!("{}", solution.to_string()),
        Err(e) => eprintln!("error: {}", e),
    }
}
