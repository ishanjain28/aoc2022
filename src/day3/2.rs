#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> Vec<(&str, &str, &str)> {
    let lines: Vec<&str> = input.trim().lines().collect();

    lines
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution(input: Vec<(&str, &str, &str)>) -> usize {
    let mut score = 0;

    for (a, b, c) in input.into_iter() {
        let ai = find_items(a);
        let bi = find_items(b);
        let ci = find_items(c);

        for i in 0..128 {
            if ai[i] & bi[i] & ci[i] {
                let c = i as u8 as char;

                let lscore = if ('a'..='z').contains(&c) {
                    i - 97 + 1
                } else {
                    i - 65 + 27
                };

                score += lscore;
            }
        }
    }

    score
}

fn find_items(ip: &str) -> [bool; 128] {
    let mut out = [false; 128];

    for c in ip.bytes() {
        out[c as usize] = true;
    }

    out
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
