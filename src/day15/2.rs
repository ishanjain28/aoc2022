#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
struct Sensor {
    sx: i64,
    sy: i64,

    area: i64,
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
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i64);
                        let y = y
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i64);

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
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i64);
                        let y = y
                            .bytes()
                            .filter(|c| (b'0'..=b'9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i64);

                        (if xneg { -x } else { x }, if yneg { -y } else { y })
                    })
                    .unwrap();
                Sensor {
                    sx,
                    sy,
                    area: (sx - bx).abs() + (sy - by).abs(),
                }
            })
        })
        .collect()
}

fn solution(input: Vec<Sensor>, line: i64) -> i64 {
    for j in 0..=line {
        let mut i = 0;

        'out: while i <= line {
            for beacon in input.iter() {
                if inside((beacon.sx, beacon.sy), (i, j)) <= beacon.area {
                    let dy = (j - beacon.sy).abs();
                    let dx = (beacon.sx - i) + (beacon.area - dy) + 1;
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
