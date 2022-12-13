#![feature(test)]

use std::cmp::Ordering;
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug, Eq, PartialEq, Clone)]
enum Node {
    Number(u8),
    List(Vec<Node>),
}

fn parse(input: &[u8]) -> Vec<Node> {
    input
        .split(|&c| c == b'\n')
        .filter(|c| !c.is_empty())
        .map(|c| parse_line(&mut c.iter()))
        .collect()
}

fn parse_line<'a>(s: &mut impl Iterator<Item = &'a u8>) -> Node {
    let mut answer = Vec::with_capacity(5);
    let mut num = None;
    while let Some(c) = s.next() {
        match c {
            b'[' => answer.push(parse_line(s)),
            b']' => {
                if let Some(n) = num {
                    answer.push(Node::Number(n));
                }
                return Node::List(answer);
            }
            b',' => {
                if let Some(n) = num {
                    answer.push(Node::Number(n));
                }
                num = None;
            }
            v => num = Some((num.unwrap_or(0) * 10) + (*v - b'0')),
        }
    }

    Node::List(answer)
}

fn compute_score(left: &Node, right: &Node) -> Ordering {
    match (left, right) {
        (Node::List(a), Node::List(b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                match compute_score(a, b) {
                    Ordering::Equal => {}
                    v => return v,
                }
            }

            a.len().cmp(&b.len())
        }
        (Node::Number(a), Node::Number(b)) => a.cmp(b),
        (Node::Number(a), b) => compute_score(&Node::List(vec![Node::Number(*a)]), b),
        (a, Node::Number(b)) => compute_score(a, &Node::List(vec![Node::Number(*b)])),
    }
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);
        println!("{}", score);
    }
}

fn solution(mut input: Vec<Node>) -> usize {
    let a = Node::List(vec![Node::List(vec![Node::Number(2)])]);
    let b = Node::List(vec![Node::List(vec![Node::Number(6)])]);

    input.push(a.clone());
    input.push(b.clone());

    input.sort_unstable_by(compute_score);

    input.iter().position(|c| c == &a).map_or(0, |c| c + 1)
        * input.into_iter().position(|c| c == b).map_or(0, |c| c + 1)
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
