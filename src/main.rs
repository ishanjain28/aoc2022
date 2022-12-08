#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
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
            let th = input[i][j];

            let mut counts = [0, 0, 0, 0];
            for &v in input[i][0..j].iter().rev() {
                counts[0] += 1;
                if v >= th {
                    break;
                }
            }

            for &v in &input[i][j + 1..n] {
                counts[1] += 1;
                if v >= th {
                    break;
                }
            }

            for row in input[0..i].iter().rev() {
                counts[2] += 1;
                if row[j] >= th {
                    break;
                }
            }

            for row in &input[i + 1..m] {
                counts[3] += 1;
                if row[j] >= th {
                    break;
                }
            }

            answer = std::cmp::max(answer, counts[0] * counts[1] * counts[2] * counts[3]);
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
