#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug, Copy, Clone)]
struct Sensor {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
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
                    v => num = num * 10 + (v - b'0') as i64,
                }
            }
            out[i] = if is_neg { -num } else { num };

            let distance = (out[0] - out[2]).abs() + (out[1] - out[3]).abs();

            let (s, r) = rotneg45cw(out[0], out[1]);

            let (left, right) = (s - distance - 1, s + distance + 1);
            let (top, bottom) = (r - distance - 1, r + distance + 1);
            Sensor {
                left,
                right,
                top,
                bottom,
            }
        })
        .collect()
}

fn solution(input: Vec<Sensor>) -> i64 {
    let mut left = 0;
    let mut right = 0;

    'outer: for x in input.iter() {
        for y in input.iter() {
            if y.left == x.right && (x.top < y.bottom || y.top < x.bottom) {
                left = y.left;
                break 'outer;
            }
        }
    }
    'outer: for x in input.iter() {
        for y in input.iter() {
            if y.top == x.bottom && (x.left < y.right || y.left < x.right) {
                right = y.top;
                break 'outer;
            }
        }
    }

    let (fx, fy) = rotneg45ccw(left, right);
    4000000 * fx + fy
}

#[inline]
const fn rotneg45ccw(x: i64, y: i64) -> (i64, i64) {
    let a = (x + y) / 2;
    (y - a, a)
}

#[inline]
const fn rotneg45cw(x: i64, y: i64) -> (i64, i64) {
    (-x + y, x + y)
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
