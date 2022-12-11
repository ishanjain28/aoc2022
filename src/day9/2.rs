#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug)]
enum Move {
    R(u8),
    L(u8),
    U(u8),
    D(u8),
}

fn parse(input: &[u8]) -> Vec<Move> {
    input
        .trim_ascii()
        .split(|&c| c == b'\n')
        .map(|line| {
            let (a, b) = line.split_at(1);

            let b = b.iter().skip(1).fold(0, |a, x| (a * 10) + (x - b'0'));

            match &a {
                [b'R'] => Move::R(b),
                [b'L'] => Move::L(b),
                [b'U'] => Move::U(b),
                [b'D'] => Move::D(b),

                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}

fn solution(input: Vec<Move>) -> usize {
    let mut locs = [(0, 0); 10];
    let mut set: HashSet<(i32, i32)> = HashSet::with_capacity_and_hasher(3000, Default::default());
    set.insert((0, 0));

    for mmove in input {
        let (steps, (dsxh, dsyh)) = match mmove {
            Move::R(v) => (v, (1, 0)),
            Move::L(v) => (v, (-1, 0)),
            Move::U(v) => (v, (0, -1)),
            Move::D(v) => (v, (0, 1)),
        };

        let locs9 = locs[9];

        for _ in 0..steps {
            // Update Head position
            locs[0].0 += dsxh;
            locs[0].1 += dsyh;

            // One by one, Updated position of each knot
            for i in 1..10 {
                let loci = move_tail(locs[i - 1], locs[i]);
                if loci == locs[i] {
                    break;
                }
                locs[i] = loci;
            }

            if locs9 != locs[9] {
                set.insert(locs[9]);
            }
        }
    }

    set.len()
}

#[inline]
const fn move_tail((sxh, syh): (i32, i32), (sxt, syt): (i32, i32)) -> (i32, i32) {
    let dx = sxh - sxt;
    let dy = syh - syt;

    if dx.abs() == 2 || dy.abs() == 2 {
        // signum gets you 1 or -1 depending on the sign of number
        (sxt + dx.signum(), syt + dy.signum())
    } else {
        (sxt, syt)
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
