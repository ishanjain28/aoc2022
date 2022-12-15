#![feature(test)]

use std::collections::HashSet;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Copy, Clone, Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Sensor {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

fn parse(input: &[u8]) -> Vec<Sensor> {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut out = [0; 4];
            let mut i = 0;
            let mut num = 0;
            let mut is_neg = false;

            for c in line
                .iter()
                .filter(|&c| (b'0'..=b'9').contains(c) || *c == b',' || *c == b'-' || *c == b':')
            {
                match c {
                    b'-' => is_neg = true,
                    b',' | b':' => {
                        out[i] = if is_neg { -num } else { num };

                        num = 0;
                        is_neg = false;
                        i += 1;
                    }
                    v => num = num * 10 + (v - b'0') as i32,
                }
            }
            out[i] = if is_neg { -num } else { num };

            Sensor {
                sx: out[0],
                sy: out[1],
                bx: out[2],
                by: out[3],
            }
        })
        .collect()
}

fn solution(input: Vec<Sensor>, line: i32) -> usize {
    let mut ranges = Vec::with_capacity(500);

    let mut nranges: Vec<(i32, i32)> = input
        .iter()
        .filter_map(|sensor| {
            let distance = (sensor.sx - sensor.bx).abs() + (sensor.sy - sensor.by).abs();
            // Covered area reduces as it moves further away from the center
            // The covered area at line will be the manhatten distance - (distance between sensor
            // and line)
            let dy = distance - (sensor.sy - line).abs();

            if dy < 0 {
                None
            } else {
                Some((sensor.sx - dy, sensor.sx + dy))
            }
        })
        .collect();

    nranges.sort_unstable();

    for (start, end) in nranges {
        if let Some((_, le)) = ranges.last_mut() {
            if start <= *le {
                // If ranges overlap, Edit the last range to extend till the limit of last range or
                // the current range
                *le = std::cmp::max(*le, end);
            } else {
                ranges.push((start, end));
            }
        } else {
            ranges.push((start, end));
        }
    }

    let mut answer = 0;
    for (s, e) in ranges {
        answer += (e - s + 1) as usize;
    }

    answer
        - input
            .into_iter()
            .filter(|x| x.by == line)
            .map(|x| x.bx)
            .collect::<HashSet<i32>>()
            .len()
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
