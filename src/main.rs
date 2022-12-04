#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

fn parse(input: &[u8]) -> impl Iterator<Item = ((u8, u8), (u8, u8))> + '_ {
    input
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split(|&c| c == b',').flat_map(|set| {
                set.split(|&c| c == b'-').map(|num| {
                    num.get(1)
                        .map_or(num[0] - b'0', |c| (num[0] - b'0') * 10 + c - b'0')
                })
            });

            (
                (nums.next().unwrap(), nums.next().unwrap()),
                (nums.next().unwrap(), nums.next().unwrap()),
            )
        })
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution(input: impl Iterator<Item = ((u8, u8), (u8, u8))>) -> usize {
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
