#![feature(test)]

use std::collections::VecDeque;
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

struct Grid<const M: usize, const N: usize> {
    grid: [[u8; N]; M],
    start: (usize, usize),
    end: (usize, usize),
}

fn parse<const M: usize, const N: usize>(input: &[u8]) -> Grid<M, N> {
    let mut out = [[0; N]; M];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in input.split(|&c| c == b'\n').enumerate() {
        for (j, v) in row.iter().enumerate() {
            out[i][j] = match *v {
                b'E' => {
                    end = (i, j);
                    b'z'
                }
                b'S' => {
                    start = (i, j);
                    b'a'
                }
                v => v,
            };
        }
    }

    Grid {
        grid: out,
        start,
        end,
    }
}

fn main() {
    let output = parse::<5, 8>(INPUTS[0]);
    let score = solution::<5, 8>(output);
    println!("{}", score);
    let output = parse::<41, 61>(INPUTS[1]);
    let score = solution::<41, 61>(output);
    println!("{}", score);
}

fn solution<const M: usize, const N: usize>(input: Grid<M, N>) -> usize {
    let mut q = VecDeque::with_capacity(500);
    q.push_back((input.start.0, input.start.1, 0));

    let mut visited = [[false; N]; M];

    while !q.is_empty() {
        let l = q.len();

        for _ in 0..l {
            let next = q.pop_front().unwrap();
            let cheight = input.grid[next.0][next.1];

            if next.0 == input.end.0 && next.1 == input.end.1 {
                return next.2;
            }

            if visited[next.0][next.1] {
                continue;
            }
            visited[next.0][next.1] = true;

            for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let i = next.0 as i32 + x;
                let j = next.1 as i32 + y;

                if i < 0
                    || j < 0
                    || i >= M as i32
                    || j >= N as i32
                    || visited[i as usize][j as usize]
                {
                    continue;
                }

                let nheight = input.grid[i as usize][j as usize];

                if nheight - cheight <= 1 || nheight <= cheight {
                    q.push_back((i as usize, j as usize, next.2 + 1))
                }
            }
        }
    }

    0
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse::<41, 61>(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
