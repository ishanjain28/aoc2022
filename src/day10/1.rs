#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
enum Ins {
    Noop,
    Addx(i32),
}

fn parse(input: &'static str) -> Vec<Ins> {
    input
        .trim()
        .split('\n')
        .flat_map(|line| {
            let (a, b) = line.split_at(4);

            match a {
                "noop" => vec![Ins::Noop],
                "addx" => {
                    let b = b.trim();
                    let b: i32 = b.parse::<i32>().unwrap();

                    vec![Ins::Noop, Ins::Addx(b)]
                }
                _ => unreachable!(),
            }
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

fn solution(input: Vec<Ins>) -> i32 {
    let mut register = 1i32;
    let mut cycle = 0;
    let mut store = [0; 220];

    for ip in input.into_iter() {
        match ip {
            Ins::Noop => {
                store[cycle] = register;
            }
            Ins::Addx(v) => {
                store[cycle] = register;
                register += v;
            }
        }
        cycle += 1;
        if cycle >= 220 {
            break;
        }
    }

    store[20 - 1] * 20
        + store[60 - 1] * 60
        + store[100 - 1] * 100
        + store[140 - 1] * 140
        + store[180 - 1] * 180
        + store[220 - 1] * 220
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
