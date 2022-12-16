#![feature(test)]

use std::{cmp::Ordering, collections::HashMap};
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Node {
    flow: i32,
    idx: usize,
    leads_to: Vec<String>,
}

fn parse(input: &'static str) -> HashMap<String, Node> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut num = 0;
            let mut node = "".to_string();
            let mut children = vec![];
            for c in line.chars().skip(1).filter(|&c| {
                ('A'..='Z').contains(&c)
                    || ('0'..='9').contains(&c)
                    || c == ','
                    || c == ';'
                    || c == '='
            }) {
                match c {
                    'A'..='Z' => node.push(c),
                    '0'..='9' => num = num * 10 + (c as u8 - b'0') as i32,
                    '=' | ',' => {
                        children.push(node.clone());
                        node.clear();
                    }

                    _ => (),
                }
            }

            children.push(node);

            (
                children[0].clone(),
                Node {
                    flow: num,
                    idx: i,
                    leads_to: children[1..].to_vec(),
                },
            )
        })
        .collect()
}

const MAX_TIME: i32 = 26;

fn solution(input: HashMap<String, Node>) -> i32 {
    let mut new_choices = HashMap::with_capacity(500);
    let mut choices = HashMap::with_capacity(500);
    choices.insert((0, "AA".to_string(), "AA".to_string()), (0, 0u128));

    'outer: for minute in 0..MAX_TIME {
        for (k, choice) in &choices {
            let mut hpaths = input.get(&k.1).unwrap().leads_to.clone();
            hpaths.push(k.1.clone());

            let mut epaths = input.get(&k.2).unwrap().leads_to.clone();
            epaths.push(k.2.clone());

            if choice.1.count_ones() == input.len() as u32 {
                break 'outer;
            }

            for h in hpaths {
                let epaths = epaths.clone();
                for e in epaths {
                    let new_h_valve = h.clone();
                    let new_h_valve_flow = input.get(&new_h_valve).unwrap().flow;
                    let new_h_valve_idx = input.get(&new_h_valve).unwrap().idx;

                    let mut new_release = choice.0;
                    let new_e_valve = e;
                    let new_e_valve_flow = input.get(&new_e_valve).unwrap().flow;
                    let new_e_valve_idx = input.get(&new_e_valve).unwrap().idx;

                    let mut new_state = choice.1;

                    match (new_e_valve.cmp(&k.2), new_h_valve.cmp(&k.1)) {
                        (Ordering::Equal, Ordering::Equal) => {
                            if (new_state & (1 << new_h_valve_idx) < 1)
                                && (new_state & (1 << new_e_valve_idx) < 1)
                            {
                                new_release += new_h_valve_flow * (MAX_TIME - minute - 1);
                                new_state |= 1 << new_h_valve_idx;
                                if new_state & (1 << new_e_valve_idx) < 1 {
                                    new_release += new_e_valve_flow * (MAX_TIME - minute - 1);
                                    new_state |= 1 << new_e_valve_idx;
                                }
                            }
                        }
                        (_, Ordering::Equal) if new_state & (1 << new_h_valve_idx) < 1 => {
                            new_release += new_h_valve_flow * (MAX_TIME - minute - 1);
                            new_state |= 1 << new_h_valve_idx;
                        }
                        (Ordering::Equal, _) if new_state & (1 << new_e_valve_idx) < 1 => {
                            new_release += new_e_valve_flow * (MAX_TIME - minute - 1);
                            new_state |= 1 << new_e_valve_idx;
                        }

                        (_, _) => (),
                    }

                    let new_choice = (new_release, new_state);

                    new_choices.insert((new_release, new_h_valve, new_e_valve), new_choice);
                }
            }
        }

        std::mem::swap(&mut choices, &mut new_choices);
        new_choices.clear();
    }

    choices.values().map(|c| c.0).max().unwrap()
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
