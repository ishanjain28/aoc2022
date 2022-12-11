#![feature(test)]

use std::{cmp::Reverse, rc::Rc};
extern crate test;

const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Rc<Box<dyn Fn(usize) -> usize>>,
    div_by_test: usize,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &'static str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter(|c| !c.is_empty())
        .map(|set| {
            let mut lines = set.lines().skip(1);

            let sitems: Vec<usize> = lines
                .next()
                .unwrap()
                .split(',')
                .map(|c| {
                    c.chars()
                        .filter(|c| c.is_numeric())
                        .fold(0, |a, x| (a * 10) + (x as u8 - b'0') as usize)
                })
                .collect();

            let op = lines.next().unwrap();
            let nop = op
                .bytes()
                .filter(|c| (b'0'..=b'9').contains(c))
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            let op = move |old: usize| -> usize {
                if op.contains("old * old") {
                    old * old
                } else if op.contains("old +") {
                    old + nop
                } else if op.contains("old *") {
                    old * nop
                } else {
                    unreachable!()
                }
            };

            let test = lines
                .next()
                .unwrap()
                .bytes()
                .filter(|c| (b'0'..=b'9').contains(c))
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            let true_result = lines
                .next()
                .unwrap()
                .bytes()
                .filter(|c| (b'0'..=b'9').contains(c))
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            let false_result = lines
                .next()
                .unwrap()
                .bytes()
                .filter(|c| (b'0'..=b'9').contains(c))
                .fold(0, |a, x| (a * 10) + (x - b'0') as usize);

            Monkey {
                items: sitems,
                operation: Rc::new(Box::new(op)),
                div_by_test: test,
                if_true: true_result,
                if_false: false_result,
            }
            //
        })
        .collect()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}

fn solution(mut input: Vec<Monkey>) -> usize {
    let mlen = input.len();
    let mut activity = vec![0; mlen];

    for _ in 0..20 {
        for i in 0..mlen {
            let monkey = &input[i].clone();
            let ilen = monkey.items.len();

            for j in 0..ilen {
                let item = monkey.items[j];

                let newwlevel = (monkey.operation)(item);
                let calmed_down_level = newwlevel / 3;

                if calmed_down_level % monkey.div_by_test == 0 {
                    input[monkey.if_true].items.push(calmed_down_level);
                } else {
                    input[monkey.if_false].items.push(calmed_down_level);
                }

                activity[i] += 1;
            }
            input[i].items.clear();
        }
    }

    activity.select_nth_unstable_by_key(1, |c| Reverse(*c));

    activity[0] * activity[1]
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
