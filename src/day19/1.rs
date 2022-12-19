#![feature(test)]
extern crate test;

use std::collections::{BinaryHeap, HashSet};

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> Vec<Vec<u16>> {
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

                out.push(set.iter().fold(0, |a, x| (a * 10) + (x - b'0') as u16));
            }

            out
        })
        .collect()
}

fn solution(input: Vec<Vec<u16>>) -> u16 {
    input.iter().map(|ip| dfs(ip) * ip[0]).sum()
}

const UPTO_MINUTE: u16 = 24;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,

    ore_miners: u16,
    clay_miners: u16,
    obsidian_miners: u16,
    geode_miners: u16,

    minute: u16,
}

fn dfs(costs: &[u16]) -> u16 {
    let maximum_ore = costs[1].max(costs[2]).max(costs[3]).max(costs[5]);
    let maximum_clay = costs[4];
    let maximum_obsidian = costs[6];

    let mut map = [0; UPTO_MINUTE as usize];
    let mut answer = 0;
    let mut set = HashSet::with_capacity(100000);
    let mut stack = Vec::with_capacity(100000);
    stack.push(State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_miners: 1,
        clay_miners: 0,
        obsidian_miners: 0,
        geode_miners: 0,
        minute: 0,
    });

    while let Some(ls) = stack.pop() {
        if ls.minute >= UPTO_MINUTE {
            answer = std::cmp::max(answer, ls.geodes);
            continue;
        }

        // Break off if there is not enough time to catch up to current leader
        // given the time left
        if ls.geodes < map[ls.minute as usize] {
            continue;
        } else {
            map[ls.minute as usize] = std::cmp::max(map[ls.minute as usize], ls.geodes)
        }

        if set.contains(&ls) {
            continue;
        } else {
            set.insert(ls);
        }

        for min in ls.minute..=UPTO_MINUTE {
            let mut built_a_robot = false;
            {
                // Geode Robot
                let [ore, _, obsidian] = find_cost(costs, 3);
                let count = std::cmp::min(ls.ore / ore, ls.obsidian / obsidian);
                let remain_ia = ls.ore - (count * ore);
                let remain_ic = ls.obsidian - (count * obsidian);

                stack.push(State {
                    ore: remain_ia + ls.ore_miners,
                    clay: ls.clay + ls.clay_miners,
                    obsidian: remain_ic + ls.obsidian_miners,
                    geodes: ls.geodes + ls.geode_miners,
                    minute: min + 1,
                    geode_miners: ls.geode_miners + count,

                    ..ls
                });
                if count > 0 {
                    continue;
                }
            }

            {
                // Obsidian Miners
                if ls.obsidian_miners < maximum_obsidian && ls.clay_miners < maximum_clay {
                    let [ore, clay, _] = find_cost(costs, 2);
                    let mut ia = ls.ore;
                    let mut ib = ls.clay;

                    while ia >= ore && ib >= clay {
                        ia -= ore;
                        ib -= clay;

                        stack.push(State {
                            ore: ia + ls.ore_miners,
                            clay: ib + ls.clay_miners,
                            obsidian: ls.obsidian + ls.obsidian_miners,
                            geodes: ls.geodes + ls.geode_miners,
                            obsidian_miners: ls.obsidian_miners + 1,
                            minute: min + 1,

                            ..ls
                        });
                    }
                    built_a_robot = true;
                }
            }

            {
                if ls.clay_miners < maximum_clay {
                    let [ore, _, _] = find_cost(costs, 1);
                    let mut ia = ls.ore;

                    while ia >= ore {
                        ia -= ore;

                        stack.push(State {
                            ore: ia + ls.ore_miners,
                            clay: ls.clay + ls.clay_miners,
                            obsidian: ls.obsidian + ls.obsidian_miners,
                            geodes: ls.geodes + ls.geode_miners,
                            clay_miners: ls.clay_miners + 1,
                            minute: min + 1,

                            ..ls
                        });
                    }
                    built_a_robot = true;
                }
            }

            {
                // Make robots
                if ls.ore_miners < maximum_ore {
                    let [ore, _, _] = find_cost(costs, 0);
                    let mut ia = ls.ore;

                    while ia >= ore {
                        ia -= ore;

                        stack.push(State {
                            ore: ia + ls.ore_miners,
                            clay: ls.clay + ls.clay_miners,
                            obsidian: ls.obsidian + ls.obsidian_miners,
                            geodes: ls.geodes + ls.geode_miners,
                            ore_miners: ls.ore_miners + 1,
                            minute: min + 1,

                            ..ls
                        });
                    }
                    built_a_robot = true;
                }
            }

            {
                // Go on without making a robot
                if !built_a_robot {
                    stack.push(State {
                        ore: ls.ore + ls.ore_miners,
                        clay: ls.clay + ls.clay_miners,
                        obsidian: ls.obsidian + ls.obsidian_miners,
                        geodes: ls.geodes + ls.geode_miners,
                        minute: min + 1,

                        ..ls
                    });
                }
            }
        }
    }

    answer
}

#[inline]
const fn find_cost(costs: &[u16], i: u16) -> [u16; 3] {
    match i {
        0 => [costs[1], 0, 0],
        1 => [costs[2], 0, 0],
        2 => [costs[3], costs[4], 0],
        3 => [costs[5], 0, costs[6]],
        _ => unreachable!(),
    }
}

fn main() {
    for input in INPUTS.iter().take(1) {
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
