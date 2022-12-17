#![feature(test)]

use std::collections::HashSet;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Eq)]
enum Moves {
    Left,
    Right,
}

fn parse(input: &'static str) -> Vec<Moves> {
    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'>' => Moves::Right,
                b'<' => Moves::Left,
                _ => unreachable!(),
            })
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Rock {
    Horizontal,
    Vertical,
    Plus,
    L,
    Box,
}

impl Rock {
    pub fn points(rock: &Rock, height: i32) -> Vec<(i32, i32)> {
        match rock {
            Rock::Horizontal => vec![(height, 2), (height, 3), (height, 4), (height, 5)],
            Rock::Vertical => vec![
                (height, 2),
                (height + 1, 2),
                (height + 2, 2),
                (height + 3, 2),
            ],
            Rock::Plus => vec![
                (height, 3),
                (height + 1, 2),
                (height + 1, 3),
                (height + 1, 4),
                (height + 2, 3),
            ],
            Rock::L => vec![
                (height, 2),
                (height, 3),
                (height, 4),
                (height + 1, 4),
                (height + 2, 4),
            ],
            Rock::Box => vec![(height, 2), (height, 3), (height + 1, 2), (height + 1, 3)],
        }
    }

    pub fn move_left(points: &mut [(i32, i32)], frozen_points: &HashSet<(i32, i32)>) {
        if points
            .iter()
            .any(|(a, c)| frozen_points.contains(&(*a, *c - 1)) || *c == 0)
        {
            return;
        }

        for (_, c) in points.iter_mut() {
            *c -= 1;
        }
    }

    pub fn move_right(points: &mut [(i32, i32)], frozen_points: &HashSet<(i32, i32)>) {
        if points
            .iter()
            .any(|(a, c)| frozen_points.contains(&(*a, *c + 1)) || *c == 6)
        {
            return;
        }

        for (_, c) in points.iter_mut() {
            *c += 1;
        }
    }

    pub fn move_down(points: &mut [(i32, i32)], frozen_points: &HashSet<(i32, i32)>) -> bool {
        if points
            .iter()
            .any(|(a, c)| frozen_points.contains(&(*a - 1, *c)) || *a == 0)
        {
            return false;
        }

        for (a, _) in points.iter_mut() {
            *a -= 1;
        }

        true
    }
}

const ROCKS: [Rock; 5] = [
    Rock::Horizontal,
    Rock::Plus,
    Rock::L,
    Rock::Vertical,
    Rock::Box,
];

#[derive(Debug)]
struct Grid {
    highest_point: i32,
    frozen_points: HashSet<(i32, i32)>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            highest_point: 0,
            frozen_points: HashSet::with_capacity_and_hasher(5000000, Default::default()),
        }
    }

    pub fn add_points(&mut self, points: Vec<(i32, i32)>) {
        let highest = *points.iter().map(|(a, _)| a).max().unwrap();

        self.highest_point = std::cmp::max(self.highest_point, highest + 1);

        self.frozen_points.extend(points.into_iter())
    }
}

fn solution(input: Vec<Moves>) -> usize {
    let mut grid = Grid::new();

    let mut deltas = vec![0; 5000];

    let mut input = input.iter().cycle();

    for (k, rock) in ROCKS.iter().cycle().take(5000).enumerate() {
        let mut height = grid.highest_point + 3;

        let mut rockp = Rock::points(rock, height);

        for wind in input.by_ref() {
            match wind {
                Moves::Left => Rock::move_left(&mut rockp, &grid.frozen_points),
                Moves::Right => Rock::move_right(&mut rockp, &grid.frozen_points),
            }

            if Rock::move_down(&mut rockp, &grid.frozen_points) {
                height -= 1;
            } else {
                break;
            }
        }

        let old_height = grid.highest_point;
        grid.add_points(rockp);

        deltas[k] = grid.highest_point - old_height;
    }

    let pattern_length = 1000;
    let mut found_plen = 0;
    let d = &deltas[pattern_length..];
    for plen in 1..=d.len() / 2 {
        let pattern = &d[0..plen];
        let mut found = true;

        for i in 0..d.len() - plen {
            if d[i + plen] != pattern[i % plen] {
                found = false;
                break;
            }
        }
        if found {
            found_plen = plen;
            break;
        }
    }

    let pattern = &d[0..found_plen];
    let pattern_sum = pattern.iter().sum::<i32>();

    let iter_count = 1000000000000usize;

    let mut idx = 0;

    for i in 0..deltas.len() {
        if &deltas[i..i + found_plen] == pattern {
            idx = i;
            break;
        }
    }

    let ideltas = &deltas[0..idx];
    let ideltassum = ideltas.iter().sum::<i32>();

    let num_patterns = (iter_count - ideltas.len()) / pattern.len();
    let num_leftover = (iter_count - ideltas.len()) % pattern.len();

    let leftoversum = pattern[0..num_leftover].iter().sum::<i32>();

    ideltassum as usize + pattern_sum as usize * num_patterns + leftoversum as usize
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
