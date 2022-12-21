#![feature(box_patterns)]
#![feature(test)]

use std::collections::HashMap;
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Clone, Copy)]
enum Operation {
    Divide,
    Multiply,
    Add,
    Subtract,
}

#[derive(Debug)]
struct Monkey {
    operand1: Operand,
    op: Option<Operation>,
    operand2: Option<Operand>,
}

#[derive(Debug)]
enum Operand {
    Number(i64),
    Reference(String),
}

fn parse(input: &'static str) -> HashMap<String, Monkey> {
    input
        .lines()
        .filter(|c| !c.is_empty())
        .flat_map(|line| {
            line.split_once(':').map(|(name, values)| {
                let mut tokens = values.trim().splitn(3, ' ');

                let operand1 = tokens.next();

                let op = tokens
                    .next()
                    .map(|token| {
                        Some(match token {
                            "*" => Operation::Multiply,
                            "/" => Operation::Divide,
                            "-" => Operation::Subtract,
                            "+" => Operation::Add,
                            _ => unreachable!(),
                        })
                    })
                    .unwrap_or(None);

                let operand2 = tokens
                    .next()
                    .map(|token| Some(Operand::Reference(token.to_string())))
                    .unwrap_or(None);

                let operand1 = if operand2.is_some() {
                    Operand::Reference(operand1.unwrap().to_string())
                } else {
                    Operand::Number(
                        operand1
                            .unwrap()
                            .bytes()
                            .fold(0, |a, x| (a * 10) + (x - b'0') as i64),
                    )
                };

                (
                    name.to_string(),
                    Monkey {
                        operand1,
                        op,
                        operand2,
                    },
                )
            })
        })
        .collect()
}

fn solution(input: HashMap<String, Monkey>) -> i64 {
    let root = input.get("root").unwrap();

    let (op1, op2) = match (&root.operand1, &root.operand2) {
        (Operand::Reference(v), Some(Operand::Reference(c))) => (v.clone(), c.clone()),
        (_, _) => unreachable!(),
    };

    let answer = 0;

    match (resolve(&input, &op1), resolve(&input, &op2)) {
        (a, Computation::Known(b)) => {
            let r = std::i64::MAX;

            let mut pmid = 0;
            for step in (0..=13).rev() {
                let mut prev = None;
                let l = pmid;
                let s = 10usize.pow(step);

                for mid in (l..=r).step_by(s) {
                    let value = rev_resolve(a.clone(), mid);

                    if let Some(p) = prev {
                        if p < b && value > b || p > b && value < b {
                            pmid = mid - s as i64;

                            break;
                        } else if p == b {
                            return mid - s as i64;
                        }
                    }
                    prev = Some(value);
                }
            }
        }
        (Computation::Known(b), a) => {
            let r = std::i64::MAX;

            let mut pmid = 0;
            for step in (0..=13).rev() {
                let mut prev = None;
                let l = pmid;
                let s = 10usize.pow(step);

                for mid in (l..=r).step_by(s) {
                    let value = rev_resolve(a.clone(), mid);

                    if let Some(p) = prev {
                        if p < b && value > b || p > b && value < b {
                            pmid = mid - s as i64;

                            break;
                        } else if p == b {
                            return mid - s as i64;
                        }
                    }
                    prev = Some(value);
                }
            }
        }

        (_, _) => unreachable!(),
    }

    answer
}

fn rev_resolve(computation: Computation, num: i64) -> i64 {
    match computation {
        Computation::Known(v) => v,
        Computation::Unknown => num,
        Computation::Lazy(op1, op, op2) => match (op1, op, op2) {
            (box Computation::Known(a), Operation::Divide, box Computation::Known(b)) => a / b,
            (box Computation::Known(a), Operation::Divide, box b) => a / rev_resolve(b, num),
            (a, Operation::Divide, box Computation::Known(b)) => rev_resolve(*a, num) / b,

            (box Computation::Known(a), Operation::Multiply, box Computation::Known(b)) => a * b,
            (box Computation::Known(a), Operation::Multiply, box b) => a * rev_resolve(b, num),
            (a, Operation::Multiply, box Computation::Known(b)) => rev_resolve(*a, num) * b,

            (box Computation::Known(a), Operation::Add, box Computation::Known(b)) => a + b,
            (box Computation::Known(a), Operation::Add, box b) => a + rev_resolve(b, num),
            (a, Operation::Add, box Computation::Known(b)) => rev_resolve(*a, num) + b,

            (box Computation::Known(a), Operation::Subtract, box Computation::Known(b)) => a - b,
            (box Computation::Known(a), Operation::Subtract, box b) => a - rev_resolve(b, num),
            (a, Operation::Subtract, box Computation::Known(b)) => rev_resolve(*a, num) - b,

            (_, _, _) => unreachable!(),
        },
    }
}

#[derive(Debug, Clone)]
enum Computation {
    Known(i64),
    Unknown,
    Lazy(Box<Computation>, Operation, Box<Computation>),
}

fn resolve(input: &HashMap<String, Monkey>, key: &str) -> Computation {
    if let Some(v) = input.get(key) {
        match (&v.operand1, &v.op, &v.operand2) {
            (Operand::Number(v), None, _) => Computation::Known(*v),
            (Operand::Reference(op1), Some(op), Some(Operand::Reference(op2))) => {
                if op1 == "humn" {
                    Computation::Lazy(
                        Box::new(Computation::Unknown),
                        *op,
                        Box::new(resolve(input, op2)),
                    )
                } else if op2 == "humn" {
                    Computation::Lazy(
                        Box::new(resolve(input, op1)),
                        *op,
                        Box::new(Computation::Unknown),
                    )
                } else {
                    match (resolve(input, op1), resolve(input, op2)) {
                        (Computation::Known(a), Computation::Known(b)) => {
                            Computation::Known(match op {
                                Operation::Divide => a / b,
                                Operation::Multiply => a * b,
                                Operation::Add => a + b,
                                Operation::Subtract => a - b,
                            })
                        }

                        (a, b) => Computation::Lazy(Box::new(a), *op, Box::new(b)),
                    }
                }
            }

            (_, _, _) => unreachable!(),
        }
    } else {
        unreachable!()
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
