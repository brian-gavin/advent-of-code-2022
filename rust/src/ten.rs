enum Instruction {
    Addx(i64),
    Noop,
}

fn parse_instruction(s: String) -> impl Iterator<Item = Instruction> {
    if s == "noop" {
        vec![Instruction::Noop].into_iter()
    } else {
        vec![
            Instruction::Noop,
            Instruction::Addx(s.split_once(' ').unwrap().1.parse().unwrap()),
        ]
        .into_iter()
    }
}

pub fn solve1(input: Vec<String>) -> i64 {
    let (.., strengths) = input.into_iter().flat_map(parse_instruction).fold(
        (1i64, 1usize, vec![]),
        |(x, cycle, mut strengths), instr| {
            let x = match instr {
                Instruction::Noop => x,
                Instruction::Addx(v) => x.wrapping_add(v),
            };
            let cycle = cycle + 1;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                strengths.push(cycle as i64 * x);
            }
            (x, cycle, strengths)
        },
    );
    strengths.into_iter().sum()
}

pub fn solve2(input: Vec<String>) -> String {
    let mut draw = Vec::<char>::new();
    input
        .into_iter()
        .flat_map(parse_instruction)
        .fold((1i64, 1i64), |(x, cycle), instr| {
            let cycle_hpos = (cycle - 1) % 40;
            let pixel = if (x - 1..=x + 1).contains(&cycle_hpos) {
                '#'
            } else {
                '.'
            };
            draw.push(pixel);
            let x = match instr {
                Instruction::Noop => x,
                Instruction::Addx(v) => x.wrapping_add(v),
            };
            (x, cycle + 1)
        });
    draw.chunks(40)
        .fold(String::with_capacity(draw.len() + 10), |mut draw, line| {
            draw.extend(line.iter().map(|c| *c as char));
            draw.push('\n');
            draw
        })
}
