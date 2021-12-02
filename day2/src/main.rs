use std::str::FromStr;

enum Op {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some(("forward", x)) => Ok(Op::Forward(x.parse().unwrap())),
            Some(("up", x)) => Ok(Op::Up(x.parse().unwrap())),
            Some(("down", x)) => Ok(Op::Down(x.parse().unwrap())),
            _ => Err(())
        }
    }
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Remove newline at EoF

    let ops: Vec<Op> = input.split("\n").map(|x| Op::from_str(x).unwrap()).collect();

    let mut horizontal = 0;
    let mut depth = 0;

    for op in &ops {
        match op {
            Op::Forward(x) => horizontal += x,
            Op::Up(x) => depth -= x,
            Op::Down(x) => depth += x,
        }
    }

    let mut aim = 0;
    let mut horizontal_two = 0;
    let mut depth_two = 0;

    for op in &ops {
        match op {
            Op::Forward(x) => {
                horizontal_two += x;
                depth_two += aim * x;
            },
            Op::Up(x) => aim -= x,
            Op::Down(x) => aim += x,
        }
    }

    println!("Part one: {}", horizontal * depth);
    println!("Part two: {}", horizontal_two * depth_two);
}
