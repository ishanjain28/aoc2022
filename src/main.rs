#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::collections::HashSet;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

#[derive(Debug)]
enum Move {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

fn parse(input: &[u8]) -> impl Iterator<Item = Move> + '_ {
    input.trim_ascii().split(|&c| c == b'\n').map(|line| {
        let (a, b) = line.split_at(1);

        let b = b
            .iter()
            .skip(1)
            .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

        match &a {
            [b'R'] => Move::R(b),
            [b'L'] => Move::L(b),
            [b'U'] => Move::U(b),
            [b'D'] => Move::D(b),

            _ => unreachable!(),
        }
    })
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}

fn solution(input: impl Iterator<Item = Move>) -> usize {
    let (mut sxh, mut syh) = (0, 0);
    let (mut sxt, mut syt) = (0, 0);

    let mut set = HashSet::with_capacity(5000);

    for mmove in input {
        let (steps, (dsxh, dsyh)): (usize, (i32, i32)) = match mmove {
            Move::R(v) => (v, (1, 0)),
            Move::L(v) => (v, (-1, 0)),
            Move::U(v) => (v, (0, -1)),
            Move::D(v) => (v, (0, 1)),
        };
        for _ in 0..steps {
            sxh += dsxh;
            syh += dsyh;

            (sxt, syt) = move_tail((sxh, syh), (sxt, syt));

            set.insert((sxt, syt));
        }
    }

    set.len()
}

#[inline]
const fn move_tail((sxh, syh): (i32, i32), (sxt, syt): (i32, i32)) -> (i32, i32) {
    let dx = sxh - sxt;
    let dy = syh - syt;

    if dx.abs() == 2 || dy.abs() == 2 {
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
