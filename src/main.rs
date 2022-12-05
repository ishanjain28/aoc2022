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
        let mut output = vec![];
        for &c in l
            .iter()
            .filter(|&c| (b'0'..=b'9').contains(c) || c == &b' ')
        {
            if c == b' ' && output.last().map_or(true, |&c| c == b' ') {
                continue;
            }
            output.push(c);
        }

        if output.is_empty() {
            break;
        }

        let mut input = output.splitn(2, |&c| c == b' ');
        let e1 = input.next().map_or(0, |c| {
            c.get(1)
                .map_or(c[0] - b'0', |a| (c[0] - b'0') * 10 + a - b'0')
        });
        let mut input = input.next().unwrap().splitn(2, |&c| c == b' ');
        let e2 = input.next().map_or(0, |c| {
            c.get(1)
                .map_or(c[0] - b'0', |a| (c[0] - b'0') * 10 + a - b'0')
        });
        let e3 = input.next().map_or(0, |c| {
            c.get(1)
                .map_or(c[0] - b'0', |a| (c[0] - b'0') * 10 + a - b'0')
        });

        instructions.push(Instruction {
            count: e1 as usize,
            from: e2 as usize,
            to: e3 as usize,
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

        input.stacks[ins.to - 1].extend(crates);
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
