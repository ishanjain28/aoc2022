#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> Vec<Vec<i64>> {
    input
        .trim()
        .split("\n\n")
        .map(|set| {
            set.split('\n')
                .map(|c| c.trim())
                .map(|c| c.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);

        let max = output
            .into_iter()
            .map(|c| c.into_iter().sum::<i64>())
            .max()
            .unwrap();

        println!("{}", max);
    }
}
