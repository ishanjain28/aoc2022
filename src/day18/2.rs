#![feature(test)]

use std::collections::VecDeque;
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> Vec<[usize; 3]> {
    input
        .split(|&b| b == b'\n')
        .filter(|c| !c.is_empty())
        .map(|line| {
            let mut out = [0; 3];
            out.iter_mut()
                .zip(line.split(|&c| c == b','))
                .for_each(|(i, c)| *i = c.iter().fold(0, |a, x| (a * 10) + (x - b'0') as usize));
            out
        })
        .collect()
}

const SIZE: usize = 25;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Node {
    Water,
    Lava,
    Air,
}

fn reachable(grid: &mut [[[Node; SIZE]; SIZE]; SIZE]) {
    let mut queue = VecDeque::with_capacity(SIZE * SIZE * SIZE);
    // This assumes 0,0,0 is not enclosed lava
    queue.push_back((0, 0, 0));

    while !queue.is_empty() {
        let l = queue.len();

        for _ in 0..l {
            let node = queue.pop_front().unwrap();

            for &(x, y, z) in [
                (-1i32, 0, 0),
                (0, -1i32, 0),
                (0, 0, -1i32),
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
            ]
            .iter()
            {
                let p = x + node.0;
                let q = y + node.1;
                let r = z + node.2;

                if p < 0
                    || q < 0
                    || r < 0
                    || p >= SIZE as i32
                    || q >= SIZE as i32
                    || r >= SIZE as i32
                    || grid[p as usize][q as usize][r as usize] == Node::Lava
                {
                    continue;
                }

                if grid[p as usize][q as usize][r as usize] == Node::Water {
                    continue;
                }

                grid[p as usize][q as usize][r as usize] = Node::Water;
                queue.push_back((p, q, r))
            }
        }
    }
}

fn solution(input: Vec<[usize; 3]>) -> i32 {
    let mut answer = input.len() as i32 * 6;

    let mut grid = [[[Node::Air; SIZE]; SIZE]; SIZE];

    for ip in input {
        grid[ip[0]][ip[1]][ip[2]] = Node::Lava;
    }

    reachable(&mut grid);

    for i in 0..SIZE as i32 {
        for j in 0..SIZE as i32 {
            for k in 0..SIZE as i32 {
                match grid[i as usize][j as usize][k as usize] {
                    Node::Water | Node::Air => continue,
                    _ => (),
                }

                for &(x, y, z) in [
                    (-1i32, 0, 0),
                    (0, -1i32, 0),
                    (0, 0, -1i32),
                    (1, 0, 0),
                    (0, 1, 0),
                    (0, 0, 1),
                ]
                .iter()
                {
                    let p = x + i;
                    let q = y + j;
                    let r = z + k;

                    if p < 0
                        || q < 0
                        || r < 0
                        || p >= SIZE as i32
                        || q >= SIZE as i32
                        || r >= SIZE as i32
                    {
                        continue;
                    }

                    match grid[p as usize][q as usize][r as usize] {
                        Node::Lava | Node::Air => answer -= 1,
                        _ => (),
                    };
                }
            }
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
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
