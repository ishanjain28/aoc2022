#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Debug)]
struct Stack {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &'static str) -> Stack {
    let mut v = input.split("\n\n");

    let mut stack_input = v
        .next()
        .unwrap()
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .rev();
    let mut stacks: Vec<Vec<char>> = vec![];

    let stack_names = stack_input.next().unwrap();

    let containers: Vec<Vec<char>> = stack_input.collect();

    for (i, v) in stack_names.into_iter().enumerate() {
        if v == ' ' {
            continue;
        }
        let v: usize = (v as u8 - b'0') as usize;

        if stacks.len() <= v - 1 {
            stacks.push(vec![]);
        }

        for container in containers.iter() {
            if container[i] == ' ' {
                continue;
            }
            stacks[v - 1].push(container[i]);
        }
    }

    let instructions: Vec<Instruction> = v
        .next()
        .unwrap()
        .lines()
        .map(|line: &str| {
            let line: String = line
                .chars()
                .filter(|c| c.is_numeric() | c.is_whitespace())
                .collect();

            let numbers: Vec<u8> = line
                .split(' ')
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<u8>().unwrap())
                .collect();

            Instruction {
                count: numbers[0] as usize,
                from: numbers[1] as usize,
                to: numbers[2] as usize,
            }
        })
        .collect();

    Stack {
        instructions,
        stacks,
    }
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{}", score);
    }
}

fn solution(mut input: Stack) -> String {
    for ins in input.instructions {
        for _ in 0..ins.count {
            if let Some(v) = input.stacks[ins.from - 1].pop() {
                input.stacks[ins.to - 1].push(v);
            }
        }
    }

    input
        .stacks
        .into_iter()
        .map(|c| *c.last().unwrap())
        .collect()
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
