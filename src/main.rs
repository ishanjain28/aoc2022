#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> impl Iterator<Item = &'static str> {
    let input = input.trim().lines();

    input
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{}", score);
    }
}

fn solution(input: impl Iterator<Item = &'static str>) -> u32 {
    let mut out = Vec::with_capacity(100);
    let mut stack = Vec::with_capacity(100);

    let mut current_folder_size = 0;

    for line in input {
        match &line[..4] {
            "$ ls" | "dir " => continue,

            "$ cd" => match &line[5..6] {
                // we are supposed to match on .. but this is fine
                "." => {
                    let v = stack.pop().unwrap();
                    out.push(current_folder_size);
                    current_folder_size += v;
                }

                "/" => continue,

                _ => {
                    stack.push(current_folder_size);
                    current_folder_size = 0;
                }
            },

            _ => {
                let (size, _) = line.split_once(' ').unwrap();
                let size = size.parse::<u32>().unwrap();
                current_folder_size += size;
            }
        }
    }

    while let Some(v) = stack.pop() {
        out.push(current_folder_size);
        current_folder_size += v;
    }

    out.push(current_folder_size);

    let available_space = 70000000;
    let required_space = 30000000;
    let used_space = *out.last().unwrap();

    out.into_iter()
        .filter(|&c| c >= (required_space + used_space - available_space))
        .min()
        .unwrap()
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
