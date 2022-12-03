#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> Vec<(&str, &str, &str)> {
    let lines: Vec<&str> = input.trim().lines().collect();

    lines.chunks(3).map(|a| (a[0], a[1], a[2])).collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution<'a>(input: Vec<(&'a str, &'a str, &'a str)>) -> usize {
    let mut score = 0;

    for (a, b, c) in input {
        let ai = find_items(a);
        let bi = find_items(b);
        let ci = find_items(c);

        let mut intersect = [false; 256];

        for (i, ((x, y), z)) in ai
            .into_iter()
            .zip(bi.into_iter())
            .zip(ci.into_iter())
            .enumerate()
        {
            if x & y & z {
                intersect[i] = true;
            }
        }

        for (i, v) in intersect.into_iter().enumerate() {
            if v {
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

fn find_items(ip: &str) -> [bool; 256] {
    let mut freq = [0; 255];
    let mut out = [false; 256];

    for c in ip.bytes() {
        freq[c as usize] += 1;
    }

    for (i, v) in freq.into_iter().enumerate() {
        if v >= 1 {
            out[i] = true;
        }
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
