#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> impl Iterator<Item = ((u64, u64), (u64, u64))> {
    input.trim().lines().map(|line| {
        let (a, b) = line.split_once(',').unwrap();

        let (ai, aj) = a.split_once('-').unwrap();
        let ai = ai.parse::<u64>().unwrap();
        let aj = aj.parse::<u64>().unwrap();

        let (bi, bj) = b.split_once('-').unwrap();
        let bi = bi.parse::<u64>().unwrap();
        let bj = bj.parse::<u64>().unwrap();

        ((ai, aj), (bi, bj))
    })
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution(input: impl Iterator<Item = ((u64, u64), (u64, u64))>) -> usize {
    let mut score = 0;

    for ((a0, a1), (b0, b1)) in input {
        score += ((a0 <= b0 && a1 >= b1)
            || (b0 <= a0 && b1 >= a1)
            || (b0 >= a0 && b0 <= a1)
            || (a0 >= b0 && a0 <= b1)) as usize;
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
