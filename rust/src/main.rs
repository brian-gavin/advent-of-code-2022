use std::{env::args, fmt::Display, process};

macro_rules! generate_runner {
    ($(($n:literal, $m:ident)),+) => {
        fn run(problem: &str, input: Vec<String>) -> Result<Box<dyn Display>, String> {
            match problem {
                $(
                    concat!($n,1) => Ok(Box::new(aoc::$m::solve1(input))),
                    concat!($n,2) => Ok(Box::new(aoc::$m::solve2(input))),
                )+
                _ => Err(format!("invalid problem number: {}", problem)),
            }
        }
    };
}

generate_runner!(
    (1, one),
    (2, two),
    (3, three),
    (4, four),
    (5, five),
    (6, six),
    (7, seven),
    (8, eight),
    (9, nine),
    (10, ten),
    (11, eleven),
    (13, thirteen),
    (14, fourteen),
    (15, fifteen)
);

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
