#![feature(test)]
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

fn solution(input: Vec<[usize; 3]>) -> i32 {
    let mut answer = input.len() as i32 * 6;
    let mut grid = [[[false; SIZE]; SIZE]; SIZE];

    for ip in input.iter() {
        grid[ip[0]][ip[1]][ip[2]] = true;
    }

    for i in 0..SIZE as i32 {
        for j in 0..SIZE as i32 {
            for k in 0..SIZE as i32 {
                if !grid[i as usize][j as usize][k as usize] {
                    continue;
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

                    answer -= grid[p as usize][q as usize][r as usize] as i32;
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
