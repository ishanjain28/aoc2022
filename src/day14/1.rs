#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

const ROW_SIZE: usize = 700;
const ARR_SIZE: usize = 490000;

fn parse(input: &str) -> ([Node; ARR_SIZE], usize) {
    let mut grid = [Node::Empty; ARR_SIZE];
    let mut lowest = 0;

    for line in input.lines() {
        let coords: Vec<(usize, usize)> = line
            .split('>')
            .flat_map(|set| {
                set.split_once(',').map(|(a, b)| {
                    (
                        a.chars()
                            .filter(|c| ('0'..='9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x as u8 - b'0') as usize),
                        b.chars()
                            .filter(|c| ('0'..='9').contains(c))
                            .fold(0, |a, x| (a * 10) + (x as u8 - b'0') as usize),
                    )
                })
            })
            .collect();

        for c in coords.windows(2) {
            let start = c[0];
            let end = c[1];

            let minx = std::cmp::min(start.0, end.0);
            let maxx = std::cmp::max(start.0, end.0);

            let miny = std::cmp::min(start.1, end.1);
            let maxy = std::cmp::max(start.1, end.1);

            lowest = std::cmp::max(maxy, lowest);

            for y in miny..=maxy {
                for x in minx..=maxx {
                    grid[y * ROW_SIZE + x] = Node::Rock;
                }
            }
        }
    }

    (grid, lowest)
}

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
enum Node {
    Empty,
    Rock,
    Sand,
}

fn solution((mut input, lowest): ([Node; ARR_SIZE], usize)) -> usize {
    let mut count = 0;
    let mut stack = Vec::with_capacity(ROW_SIZE);

    let (mut px, mut py) = (500, 0);
    loop {
        if py >= lowest {
            break;
        }
        let k = (py + 1) * ROW_SIZE + px;

        (px, py) = if input[k] == Node::Empty {
            stack.push((px, py));
            (px, py + 1)
        } else if input[k - 1] == Node::Empty {
            stack.push((px, py));
            (px - 1, py + 1)
        } else if input[k + 1] == Node::Empty {
            stack.push((px, py));

            (px + 1, py + 1)
        } else {
            input[py * ROW_SIZE + px] = Node::Sand;
            count += 1;

            match stack.pop() {
                Some(v) => v,
                None => break,
            }
        };
    }

    count
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
