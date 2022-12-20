#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> Vec<i32> {
    input
        .split(|&b| b == b'\n')
        .filter(|c| !c.is_empty())
        .map(|line| {
            let isneg = line.starts_with(&[b'-']);

            let num = line
                .iter()
                .filter(|c| (b'0'..=b'9').contains(c))
                .fold(0, |a, x| (a * 10) + (x - b'0') as i32);
            if isneg {
                -num
            } else {
                num
            }
        })
        .collect()
}

fn solution(input: Vec<i32>) -> i32 {
    let l = input.len() as i32;

    let mut idxes = (0..l as usize).collect::<Vec<usize>>();

    for (i, num) in input.iter().enumerate() {
        let pos = idxes.iter().position(|&c| c == i).unwrap();
        idxes.remove(pos);
        let new_idx = (pos as i32 + num).rem_euclid(idxes.len() as i32) as usize;
        idxes.insert(new_idx, i);
    }

    let orig_zero_i = input.iter().position(|&i| i == 0).unwrap();
    let zero_i = idxes.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| input[idxes[(zero_i + i) % idxes.len()]])
        .sum()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}
#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
