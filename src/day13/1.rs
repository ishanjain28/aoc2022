#![feature(test)]

use std::cmp::Ordering;
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Clone)]
enum Node {
    Number(u8),
    List(Vec<Node>),
}

fn parse(input: &'static str) -> impl Iterator<Item = (Node, Node)> {
    input.split("\n\n").filter_map(|c| {
        c.split_once('\n').map(|(left, right)| {
            (
                parse_line(&mut left.bytes()),
                parse_line(&mut right.bytes()),
            )
        })
    })
}

fn parse_line(s: &mut impl Iterator<Item = u8>) -> Node {
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
            v => num = Some((num.unwrap_or(0) * 10) + (v - b'0')),
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

fn solution(input: impl Iterator<Item = (Node, Node)>) -> usize {
    input
        .into_iter()
        .enumerate()
        .fold(0, |a, (i, (left, right))| {
            match compute_score(&left, &right) {
                Ordering::Less => a + i + 1,
                Ordering::Greater | Ordering::Equal => a,
            }
        })
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}
