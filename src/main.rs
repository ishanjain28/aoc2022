#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> impl Iterator<Item = (Move, Outcome)> {
    input.trim().lines().map(|set| {
        let mut set = set.bytes();
        let (a, b) = (set.next().unwrap(), set.nth(1).unwrap());

        let x = match a {
            b'A' => Move::Rock,
            b'B' => Move::Paper,
            b'C' => Move::Scissors,
            _ => unreachable!(),
        };
        let y = match b {
            b'X' => Outcome::X,
            b'Y' => Outcome::Y,
            b'Z' => Outcome::Z,
            _ => unreachable!(),
        };
        (x, y)
    })
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    X = 0,
    Y = 3,
    Z = 6,
}

#[inline]
const fn calc_move(d1: Outcome, m1: Move) -> Move {
    match (d1, m1) {
        (Outcome::X, Move::Rock) => Move::Scissors,
        (Outcome::X, Move::Paper) => Move::Rock,
        (Outcome::X, Move::Scissors) => Move::Paper,
        (Outcome::Y, v) => v,
        (Outcome::Z, Move::Rock) => Move::Paper,
        (Outcome::Z, Move::Paper) => Move::Scissors,
        (Outcome::Z, Move::Scissors) => Move::Rock,
    }
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution(input: impl Iterator<Item = (Move, Outcome)>) -> i32 {
    let mut score = 0;

    for (a, b) in input {
        score += b as i32;
        score += calc_move(b, a) as i32;
    }
    score
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
