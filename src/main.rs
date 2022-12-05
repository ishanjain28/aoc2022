#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

#[derive(Debug)]
struct Stack {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &[u8]) -> Stack {
    let mut stacks = vec![];

    let mut input = input.splitn(2, |&c| c == b'\n');
    let mut line = input.next().unwrap();
    let mut rest = input.next().unwrap();

    while rest[0] != b'\n' {
        let mut i = 1;

        while (3 * (i - 1) + i) < line.len() {
            if stacks.len() < i {
                stacks.push(vec![]);
            }

            if (b'A'..=b'Z').contains(&line[3 * (i - 1) + i]) {
                stacks[i - 1].push(line[3 * (i - 1) + i]);
            }

            i += 1;
        }

        let mut temp = rest.splitn(2, |&c| c == b'\n');
        line = temp.next().unwrap();
        rest = temp.next().unwrap();
    }
    rest = &rest[1..];
    let stacks: Vec<Vec<u8>> = stacks
        .into_iter()
        .map(|c| c.into_iter().rev().collect())
        .collect();

    let mut input = rest.splitn(2, |&c| c == b'\n');
    let mut line = input.next();
    let mut rest = input.next();

    let mut instructions = vec![];

    while let Some(l) = line {
        let l: Vec<u8> = l
            .iter()
            .filter(|&c| (b'0'..=b'9').contains(c) || c == &b' ')
            .cloned()
            .collect();

        if l.is_empty() {
            break;
        }

        let numbers: Vec<u8> = l
            .split(|&c| c == b' ')
            .filter(|c| !c.is_empty())
            .map(|c| {
                c.get(1)
                    .map_or(c[0] - b'0', |a| (c[0] - b'0') * 10 + a - b'0')
            })
            .collect();

        instructions.push(Instruction {
            count: numbers[0] as usize,
            from: numbers[1] as usize,
            to: numbers[2] as usize,
        });

        if let Some(r) = rest {
            let mut input = r.splitn(2, |&c| c == b'\n');
            line = input.next();
            rest = input.next();
        } else {
            break;
        }
    }

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
        let l = input.stacks[ins.from - 1].len();

        let crates: Vec<u8> = input.stacks[ins.from - 1]
            .drain(l.saturating_sub(ins.count)..l)
            .collect();

        for c in crates {
            input.stacks[ins.to - 1].push(c);
        }
    }

    input
        .stacks
        .into_iter()
        .filter(|c| !c.is_empty())
        .map(|c| *c.last().unwrap())
        .map(|c| c as char)
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
