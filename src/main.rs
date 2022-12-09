#![feature(byte_slice_trim_ascii)]
#![feature(test)]
use std::time::Duration;

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

type Knot = u16;

const EMPTY: Knot = 0;
const POUND: Knot = 1 << 1;
const HEAD: Knot = 1 << 2;
const ONE: Knot = 1 << 3;
const TWO: Knot = 1 << 4;
const THREE: Knot = 1 << 5;
const FOUR: Knot = 1 << 6;
const FIVE: Knot = 1 << 7;
const SIX: Knot = 1 << 8;
const SEVEN: Knot = 1 << 9;
const EIGHT: Knot = 1 << 10;
const NINE: Knot = 1 << 11;

#[inline]
const fn knot_from_idx(i: usize) -> Knot {
    match i {
        0 => ONE,
        1 => TWO,
        2 => THREE,
        3 => FOUR,
        4 => FIVE,
        5 => SIX,
        6 => SEVEN,
        7 => EIGHT,
        8 => NINE,
        _ => unreachable!(),
    }
}

#[inline]
fn contains_knot(k1: &Knot, k2: Knot) -> bool {
    k1 & k2 > 0
}

fn set(grid: &mut [[Knot; GRIDX]; GRIDY], (ix, iy): (usize, usize), knot: Knot) {
    match grid[iy][ix] {
        EMPTY | POUND | HEAD => grid[iy][ix] = knot,
        _ => grid[iy][ix] |= knot,
    }
}

fn unset(grid: &mut [[Knot; GRIDX]; GRIDY], (ix, iy): (usize, usize), knot: Knot) {
    grid[iy][ix] &= !knot;
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

const GRIDX: usize = 300;
const GRIDY: usize = 350;

fn solution(input: impl Iterator<Item = Move>) -> usize {
    let mut grid = [[EMPTY; GRIDX]; GRIDY];
    let sx = 50;
    let sy = 300;
    grid[sy][sx] = HEAD | ONE | TWO | THREE | FOUR | FIVE | SIX | SEVEN | EIGHT | NINE;
    let (mut sxh, mut syh) = (sx, sy);
    let (mut sxt, mut syt) = (sx, sy);

    let mut tmoves = [[false; GRIDX]; GRIDY];

    for mmove in input {
        unset(&mut grid, (sxh, syh), HEAD);
        let (steps, (dsxh, dsyh)): (usize, (i32, i32)) = match mmove {
            Move::R(v) => (v, (1, 0)),
            Move::L(v) => (v, (-1, 0)),
            Move::U(v) => (v, (0, -1)),
            Move::D(v) => (v, (0, 1)),
        };
        for _ in 0..steps {
            unset(&mut grid, (sxh, syh), HEAD);
            sxh = (sxh as i32 + dsxh) as usize;
            syh = (syh as i32 + dsyh) as usize;
            set(&mut grid, (sxh, syh), HEAD);

            (sxt, syt) = move_tail((sxh, syh), (sxt, syt), &mut grid, knot_from_idx(0));

            tmoves[syt][sxt] = true;
        }
    }
    tmoves
        .into_iter()
        .flat_map(|c| c.into_iter().map(|a| a as usize))
        .sum()
}

fn print_grid(grid: &[[Knot; GRIDX]; GRIDY]) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut out = String::new();

    for row in grid.iter() {
        out.push_str(&row.iter().map(print_knot).collect::<String>());
        out.push('\n');
    }

    println!("{}", out);

    std::thread::sleep(Duration::from_millis(10));
}

fn print_knot(c: &Knot) -> char {
    match c {
        &HEAD => 'H',
        &ONE => '1',
        &TWO => '2',
        &THREE => '3',
        &FOUR => '4',
        &FIVE => '5',
        &SIX => '6',
        &SEVEN => '7',
        &EIGHT => '8',
        &NINE => '9',
        v if contains_knot(v, HEAD) => 'H',
        v if contains_knot(v, NINE) => '9',
        v if contains_knot(v, EIGHT) => '8',
        v if contains_knot(v, SEVEN) => '7',
        v if contains_knot(v, SIX) => '6',
        v if contains_knot(v, FIVE) => '5',
        v if contains_knot(v, FOUR) => '4',
        v if contains_knot(v, THREE) => '3',
        v if contains_knot(v, TWO) => '2',
        v if contains_knot(v, ONE) => '1',
        &POUND => '#',
        _ => '.',
    }
}

#[inline]
const fn knot_head(c: Knot) -> Knot {
    match c {
        EMPTY | HEAD | POUND => unreachable!(),
        ONE => HEAD,
        TWO => ONE,
        THREE => TWO,
        FOUR => THREE,
        FIVE => FOUR,
        SIX => FIVE,
        SEVEN => SIX,
        EIGHT => SEVEN,
        NINE => EIGHT,
        _ => unreachable!(),
    }
}

fn move_tail(
    (sxh, syh): (usize, usize),
    (sxt, syt): (usize, usize),
    grid: &mut [[Knot; GRIDX]; GRIDY],
    c: Knot,
) -> (usize, usize) {
    if sxh == sxt && syh == syt {
        return (sxt, syt);
    }

    // Present at a diagonal? Return
    // Attached to Head? Return
    if [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .map(|(a, b)| grid[(syt as i32 + a) as usize][(sxt as i32 + b) as usize])
    .any(|x| contains_knot(&x, knot_head(c)))
    {
        return (sxt, syt);
    }

    match ((sxh, syh), (sxt, syt)) {
        ((sxh, syh), (sxt, syt)) if syh == syt => {
            let dx: i32 = if sxh < sxt { -1 } else { 1 };

            unset(grid, (sxt, syt), c);
            set(grid, ((sxt as i32 + dx) as usize, syt), c);

            ((sxt as i32 + dx) as usize, syt)
        }

        ((sxh, syh), (sxt, syt)) if sxh == sxt => {
            let dx: i32 = if syh < syt { -1 } else { 1 };
            unset(grid, (sxt, syt), c);
            set(grid, (sxt, (syt as i32 + dx) as usize), c);
            (sxt, (syt as i32 + dx) as usize)
        }
        ((sxh, syh), (sxt, syt)) if syh < syt && sxh > sxt => {
            unset(grid, (sxt, syt), c);
            set(grid, (sxt + 1, syt - 1), c);

            (sxt + 1, syt - 1)
        }
        ((sxh, syh), (sxt, syt)) if syh < syt && sxh <= sxt => {
            unset(grid, (sxt, syt), c);
            set(grid, (sxt - 1, syt - 1), c);

            (sxt - 1, syt - 1)
        }
        ((sxh, syh), (sxt, syt)) if syh >= syt && sxh > sxt => {
            unset(grid, (sxt, syt), c);
            set(grid, (sxt + 1, syt + 1), c);

            (sxt + 1, syt + 1)
        }
        ((sxh, syh), (sxt, syt)) if syh >= syt && sxh <= sxt => {
            unset(grid, (sxt, syt), c);
            set(grid, (sxt - 1, syt + 1), c);

            (sxt - 1, syt + 1)
        }

        _ => (0, 0),
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
