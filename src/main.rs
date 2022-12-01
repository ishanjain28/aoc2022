use std::cmp::Reverse;

const INPUTS: [&str; 2] = [
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

        let mut sets: Vec<i64> = output
            .into_iter()
            .map(|c| c.into_iter().sum::<i64>())
            .collect();

        sets.select_nth_unstable_by_key(2, |c| Reverse(*c));

        println!("{}", sets.into_iter().take(3).sum::<i64>());
    }
}
