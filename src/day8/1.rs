#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .trim_ascii()
        .split(|&c| c == b'\n')
        .map(|line| line.iter().map(|b| b - b'0').collect())
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{}", score);
    }
}

fn solution(input: Vec<Vec<u8>>) -> usize {
    let m = input.len();
    let n = input[0].len();

    let mut answer = 0;

    for i in 0..m {
        for j in 0..n {
            match (i, j) {
                (0, _) => answer += 1,
                (_, 0) => answer += 1,
                (_, j) if j == n - 1 => answer += 1,
                (i, _) if i == m - 1 => answer += 1,

                _ => {
                    let th = input[i][j];
                    // l to r check
                    if input[i][0..j].iter().all(|&h| h < th) {
                        answer += 1;
                        continue;
                    }

                    // r to l check
                    if input[i][j + 1..n].iter().all(|&h| h < th) {
                        answer += 1;
                        continue;
                    }

                    let mut visible = true;
                    for row in &input[0..i] {
                        if row[j] >= th {
                            visible = false;
                            break;
                        }
                    }

                    if visible {
                        answer += 1;
                        continue;
                    }

                    visible = true;
                    for row in &input[i + 1..m] {
                        if row[j] >= th {
                            visible = false;
                            break;
                        }
                    }
                    if visible {
                        answer += 1;
                    }
                }
            }
        }
    }

    answer
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
