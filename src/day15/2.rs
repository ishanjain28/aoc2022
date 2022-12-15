#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug)]
struct Sensor {
    sx: i64,
    sy: i64,

    area: i64,
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

            Sensor {
                sx: out[0],
                sy: out[1],
                area: (out[0] - out[2]).abs() + (out[1] - out[3]).abs(),
            }
        })
        .collect()
}

fn solution(input: Vec<Sensor>, line: i64) -> i64 {
    for j in 0..=line {
        let mut i = 0;

        'out: while i <= line {
            for beacon in input.iter() {
                if inside((beacon.sx, beacon.sy), (i, j)) <= beacon.area {
                    let vgap = (j - beacon.sy).abs();
                    let dx = (beacon.sx - i) + (beacon.area - vgap) + 1;
                    i += dx;

                    continue 'out;
                }
            }

            return i * 4000000 + j;
        }
    }

    0
}

#[inline]
const fn inside((sx, sy): (i64, i64), (px, py): (i64, i64)) -> i64 {
    (sx - px).abs() + (sy - py).abs()
}

fn main() {
    let output = parse(INPUTS[0]);
    let score = solution(output, 20);
    println!("{}", score);
    let output = parse(INPUTS[1]);
    let score = solution(output, 4000000);
    println!("{}", score);
}
#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input, 4000000);
        test::black_box(result);
    })
}
