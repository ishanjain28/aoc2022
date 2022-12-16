#![feature(test)]

use std::collections::HashMap;

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
                ('A'..='Z').contains(&c) || ('0'..='9').contains(&c) || c == ',' || c == '='
            }) {
                match c {
                    'A'..='Z' => {
                        node.push(c);
                    }
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

const MAX_TIME: i32 = 30;

fn solution(input: HashMap<String, Node>) -> i32 {
    let mut choices = HashMap::with_capacity(50);
    choices.insert((0, "AA".to_string()), (0, 0, 0u128));

    for minute in 0..MAX_TIME {
        let mut new_choices = HashMap::with_capacity(50);

        for (k, choice) in choices {
            let new_valve = &k.1;
            let ip = input.get(new_valve).unwrap();
            let new_valve_idx = ip.idx;

            let new_time = choice.0 + 1;
            let mut new_release = choice.1;

            for path in ip.leads_to.iter() {
                let new_choice = (new_time, new_release, choice.2);

                new_choices.insert((new_release, path.to_string()), new_choice);
            }

            if ip.flow > 0 && choice.2 & (1 << new_valve_idx) < 1 {
                new_release += ip.flow * (MAX_TIME - minute - 1);

                let new_choice = (new_time, new_release, choice.2 | (1 << new_valve_idx));

                new_choices.insert((new_release, new_valve.to_string()), new_choice);
            }
        }

        choices = new_choices;
    }

    choices.values().map(|c| c.1).max().unwrap()
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
