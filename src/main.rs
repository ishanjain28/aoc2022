#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

#[derive(Debug)]
enum Ins {
    Noop,
    Addx(i32),
}

fn parse(input: &[u8]) -> Vec<Ins> {
    input
        .split(|&c| c == b'\n')
        .filter(|c| !c.is_empty())
        .map(|line| match &line[0..4] {
            [b'n', b'o', b'o', b'p'] => Ins::Noop,
            [b'a', b'd', b'd', b'x'] => {
                let is_neg = line[5] == b'-';
                let b: i32 = line[5..]
                    .iter()
                    .filter(|&&c| (b'0'..=b'9').contains(&c))
                    .fold(0, |a, &x| (a * 10) + (x - b'0') as i32);

                Ins::Addx(if is_neg { -b } else { b })
            }
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}

fn solution(input: Vec<Ins>) -> String {
    let mut register = 1i32;
    let mut cycle = 0;
    let mut line: u64 = 0;
    let mut sprite: u64 = 0b111 << 61;

    let mut answer = Vec::with_capacity(10);

    let mut i = 0;
    for ip in input.into_iter() {
        match ip {
            Ins::Noop => {
                if sprite & ((1 << 63) >> i) > 0 {
                    line |= (1 << 63) >> i;
                }

                cycle += 1;
                i += 1;

                if cycle % 40 == 0 {
                    answer.push(line);
                    line = 0;
                    i = 0;
                }
            }
            Ins::Addx(v) => {
                // Noop
                if sprite & ((1 << 63) >> i) > 0 {
                    line |= (1 << 63) >> i;
                }

                cycle += 1;
                i += 1;

                if cycle % 40 == 0 {
                    answer.push(line);
                    line = 0;
                    i = 0;
                }

                // Add
                if sprite & ((1 << 63) >> i) > 0 {
                    line |= (1 << 63) >> i;
                }

                register += v;
                sprite = (0b111 << 61) >> (register - 1);

                cycle += 1;
                i += 1;

                if cycle % 40 == 0 {
                    answer.push(line);
                    line = 0;
                    i = 0;
                }
            }
        }
    }

    let mut output = String::with_capacity(40 * 10);

    for row in answer {
        for i in 0..40 {
            if row & ((1 << 63) >> i) > 0 {
                output.push('\u{2588}');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    output
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
