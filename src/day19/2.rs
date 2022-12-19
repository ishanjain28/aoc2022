#![feature(test)]
extern crate test;
use std::collections::HashMap;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> Vec<Vec<usize>> {
    input
        .split(|&b| b == b'\n')
        .filter(|c| !c.is_empty())
        .map(|line| {
            let mut out = vec![];
            for set in line.split(|&c| c == b' ') {
                let set: Vec<u8> = set
                    .iter()
                    .filter(|&c| (b'0'..=b'9').contains(c))
                    .cloned()
                    .collect();

                if set.is_empty() {
                    continue;
                }

                out.push(set.iter().fold(0, |a, x| (a * 10) + (x - b'0') as usize));
            }

            out
        })
        .collect()
}

fn solution(input: Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .take(3)
        .map(|ip| {
            let mut memo = HashMap::new();
            solve(&mut memo, ip, (0, 0, 0, 0), (1, 0, 0, 0), 0)
        })
        .product()
}

type Store = (usize, usize, usize, usize);

const UPTO_MINUTE: usize = 32;

fn solve(
    memo: &mut HashMap<(usize, Store, Store), usize>,
    costs: &[usize],
    (mut ia, mut ib, ic, id): Store,
    (ra, rb, rc, rd): Store,
    m: usize,
) -> usize {
    if m >= UPTO_MINUTE {
        return id;
    }

    if let Some(v) = memo.get(&(m, (ia, ib, ic, id), (ra, rb, rc, rd))) {
        return *v;
    }

    let maximum_ore = std::cmp::max(
        std::cmp::max(costs[1], costs[2]),
        std::cmp::max(costs[3], costs[5]),
    );
    let maximum_clay = costs[4];
    let maximum_obsidian = costs[6];

    let mut answer = 0;
    for min in m..=UPTO_MINUTE {
        let oia = ia;
        let oib = ib;

        let mut built_a_robot = false;

        // Geode robot
        let [ore, _, obsidian] = find_cost(costs, 3);
        let count = std::cmp::min(ia / ore, ic / obsidian);
        let remain_ia = ia - (count * ore);
        let remain_ic = ic - (count * obsidian);

        if count > 0 {
            built_a_robot = true;
        }

        answer = std::cmp::max(
            answer,
            solve(
                memo,
                costs,
                (remain_ia + ra, ib + rb, remain_ic + rc, id + rd),
                (ra, rb, rc, rd + count),
                min + 1,
            ),
        );

        if built_a_robot {
            continue;
        }

        // Obsidian robot
        if rc < maximum_obsidian {
            let [ore, clay, _] = find_cost(costs, 2);
            while ia >= ore && ib >= clay {
                ia -= ore;
                ib -= clay;
                answer = std::cmp::max(
                    answer,
                    solve(
                        memo,
                        costs,
                        (ia + ra, ib + rb, ic + rc, id + rd),
                        (ra, rb, rc + 1, rd),
                        min + 1,
                    ),
                );
            }
            built_a_robot = true;
            ia = oia;
            ib = oib;
        }

        if rb < maximum_clay {
            let [ore, _, _] = find_cost(costs, 1);
            while ia >= ore {
                ia -= ore;
                answer = std::cmp::max(
                    answer,
                    solve(
                        memo,
                        costs,
                        (ia + ra, ib + rb, ic + rc, id + rd),
                        (ra, rb + 1, rc, rd),
                        min + 1,
                    ),
                );
            }
            ia = oia;
            built_a_robot = true;
        }

        // Make robots
        if ra < maximum_ore {
            let [ore, _, _] = find_cost(costs, 0);
            while ia >= ore {
                ia -= ore;
                answer = std::cmp::max(
                    answer,
                    solve(
                        memo,
                        costs,
                        (ia + ra, ib + rb, ic + rc, id + rd),
                        (ra + 1, rb, rc, rd),
                        min + 1,
                    ),
                );
            }
            ia = oia;
            built_a_robot = true;
        }

        if !built_a_robot {
            // Increase resource without doing any thing else
            answer = std::cmp::max(
                answer,
                solve(
                    memo,
                    costs,
                    (ia + ra, ib + rb, ic + rc, id + rd),
                    (ra, rb, rc, rd),
                    min + 1,
                ),
            );
        }
    }

    memo.insert((m, (ia, ib, ic, id), (ra, rb, rc, rd)), answer);

    answer
}

#[inline]
const fn find_cost(costs: &[usize], i: usize) -> [usize; 3] {
    match i {
        0 => [costs[1], 0, 0],
        1 => [costs[2], 0, 0],
        2 => [costs[3], costs[4], 0],
        3 => [costs[5], 0, costs[6]],
        _ => unreachable!(),
    }
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
