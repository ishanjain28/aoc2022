#![feature(test)]

use std::collections::HashSet;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter(|c| !c.is_empty())
        .filter_map(|line| {
            line.split_once(':').map(|(sensor, beacon)| {
                let (sx, sy) = sensor
                    .split_once(',')
                    .map(|(x, y)| {
                        let xneg = x.contains('-');
                        let yneg = y.contains('-');
                        let x = x
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i32);
                        let y = y
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i32);

                        (if xneg { -x } else { x }, if yneg { -y } else { y })
                    })
                    .unwrap();
                let (bx, by) = beacon
                    .split_once(',')
                    .map(|(x, y)| {
                        let xneg = x.contains('-');
                        let yneg = y.contains('-');
                        let x = x
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i32);
                        let y = y
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i32);

                        (if xneg { -x } else { x }, if yneg { -y } else { y })
                    })
                    .unwrap();
                Sensor { sx, sy, bx, by }
            })
        })
        .collect()
}

fn solution(mut input: Vec<Sensor>, line: i32) -> usize {
    let mut beacons = HashSet::new();
    let mut set = HashSet::new();

    input.sort_unstable_by_key(|c| c.sx);

    for Sensor { sx, sy, bx, by } in input {
        if by == line {
            beacons.insert(bx);
        }

        let area = (bx - sx).abs() + (by - sy).abs();
        for ix in sx - area..=sx + area {
            if inside((sx, sy), (ix, line)) <= area {
                set.insert(ix);
            };
        }
    }

    set.len() - beacons.len()
}

#[inline]
const fn inside((sx, sy): (i32, i32), (px, py): (i32, i32)) -> i32 {
    (sx - px).abs() + (sy - py).abs()
}

fn main() {
    let output = parse(INPUTS[0]);
    let score = solution(output, 10);
    println!("{}", score);
    let output = parse(INPUTS[1]);
    let score = solution(output, 2000000);
    println!("{}", score);
}
#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input, 2000000);
        test::black_box(result);
    })
}
