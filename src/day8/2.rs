#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse<const M: usize, const N: usize>(input: &[u8]) -> [[u8; N]; M] {
    let mut out = [[0; N]; M];
    input
        .trim_ascii()
        .split(|&c| c == b'\n')
        .enumerate()
        .for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, &v)| {
                out[i][j] = v;
            })
        });

    out
}

fn main() {
    let output = parse::<5, 5>(INPUTS[0]);
    let score = solution::<5, 5>(output);
    println!("{}", score);

    let output = parse::<99, 99>(INPUTS[1]);
    let score = solution::<99, 99>(output);
    println!("{}", score);
}

fn solution<const M: usize, const N: usize>(input: [[u8; N]; M]) -> usize {
    let mut answer = 0;

    for j in 0..N {
        for i in 0..M {
            let th = input[i][j];

            let mut counts = [0, 0, 0, 0];
            for &v in input[i][0..j].iter().rev() {
                counts[0] += 1;
                if v >= th {
                    break;
                }
            }

            for &v in &input[i][j + 1..N] {
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

            for row in &input[i + 1..M] {
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
    let input = parse::<99, 99>(INPUTS[1]);
    b.iter(|| {
        let result = solution::<99, 99>(input);
        test::black_box(result);
    })
}
