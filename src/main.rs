#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

fn parse(input: &[u8]) -> Vec<((u8, u8), (u8, u8))> {
    input
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line.splitn(2, |&c| c == b'-');
            let e1 = line.next().unwrap();

            let rest = line.next().unwrap();
            let mut line = rest.split(|&c| c == b',');
            let e2 = line.next().unwrap();

            let rest = line.next().unwrap();
            let mut line = rest.splitn(2, |&x| x == b'-');
            let e3 = line.next().unwrap();
            let e4 = line.next().unwrap();

            (
                (
                    e1.get(1)
                        .map_or(e1[0] - b'0', |c| ((e1[0] - b'0') * 10 + c - b'0')),
                    e2.get(1)
                        .map_or(e2[0] - b'0', |c| ((e2[0] - b'0') * 10 + c - b'0')),
                ),
                (
                    e3.get(1)
                        .map_or(e3[0] - b'0', |c| ((e3[0] - b'0') * 10 + c - b'0')),
                    e4.get(1)
                        .map_or(e4[0] - b'0', |c| ((e4[0] - b'0') * 10 + c - b'0')),
                ),
            )
        })
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{:?}", score);
    }
}

fn solution(input: Vec<((u8, u8), (u8, u8))>) -> usize {
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
