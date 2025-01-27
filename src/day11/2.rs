#![feature(test)]

use std::cmp::Reverse;
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    div_by_test: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug, Copy)]
enum Operation {
    MulOld,
    MulNop(usize),
    AddNop(usize),
}

impl Operation {
    fn apply(&self, v: usize) -> usize {
        match self {
            Operation::MulOld => v * v,
            Operation::MulNop(i) => v * i,
            Operation::AddNop(i) => v + i,
        }
    }
}

fn parse(input: &'static str) -> (Vec<Monkey>, usize) {
    let mut lcm = 1;
    let output = input
        .split("\n\n")
        .filter(|c| !c.is_empty())
        .map(|set| {
            let mut lines = set.lines().skip(1);

            let sitems: Vec<usize> = lines.next().unwrap()[18..]
                .split(',')
                .map(|c| {
                    c.bytes()
                        .filter(|&c| c != b' ')
                        .fold(0, |a, x| (a * 10) + (x - b'0') as usize)
                })
                .collect();

            let op = match lines.next().unwrap()[23..].split_at(1) {
                ("*", " old") => Operation::MulOld,
                ("+", v) => Operation::AddNop(
                    v[1..]
                        .bytes()
                        .fold(0, |a, x| (a * 10) + (x - b'0') as usize),
                ),
                ("*", v) => Operation::MulNop(
                    v[1..]
                        .bytes()
                        .fold(0, |a, x| (a * 10) + (x - b'0') as usize),
                ),
                (_, _) => unreachable!(),
            };

            let test = lines.next().unwrap()[21..]
                .bytes()
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            let true_result = lines.next().unwrap()[29..]
                .bytes()
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            let false_result = lines.next().unwrap()[30..]
                .bytes()
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            lcm = lcm * test / gcd(lcm, test);

            Monkey {
                items: sitems,
                operation: op,
                div_by_test: test,
                if_true: true_result,
                if_false: false_result,
            }
        })
        .collect();

    (output, lcm)
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output.0, output.1);
        println!("{}", score);
    }
}

fn solution(mut input: Vec<Monkey>, lcm: usize) -> usize {
    let mlen = input.len();
    let mut activity = [0; 8];

    for _ in 0..10000 {
        for i in 0..mlen {
            activity[i] += input[i].items.len();
            let if_true = input[i].if_true;
            let if_false = input[i].if_false;
            let operation = input[i].operation;
            let div_by_test = input[i].div_by_test;

            while let Some(item) = input[i].items.pop() {
                let newwlevel = operation.apply(item);
                let newwlevel = newwlevel % lcm;

                if newwlevel % div_by_test == 0 {
                    input[if_true].items.push(newwlevel);
                } else {
                    input[if_false].items.push(newwlevel);
                }
            }
        }
    }

    activity.select_nth_unstable_by_key(1, |c| Reverse(*c));

    activity[0] * activity[1]
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input.0, input.1);
        test::black_box(result);
    })
}

#[inline]
const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
