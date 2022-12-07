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
    let mut out = vec![];
    let mut stack = vec![];

    let mut current_folder_size = 0;

    for line in input {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = line.trim_start_matches("$ cd ");

            match dir {
                ".." => {
                    let v = stack.pop().unwrap();
                    out.push(current_folder_size);
                    current_folder_size += v;
                }

                "/" => continue,

                _ => {
                    stack.push(current_folder_size);
                    current_folder_size = 0;
                }
            }
        } else {
            let (size, _) = line.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();

            current_folder_size += size;
        }
    }

    while let Some(v) = stack.pop() {
        out.push(current_folder_size);
        current_folder_size += v;
    }

    out.push(current_folder_size);

    out.into_iter().filter(|&c| c <= 100000).sum::<u32>()
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
