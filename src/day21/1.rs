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

#[derive(Debug, Clone)]
struct Monkey {
    operand1: Operand,
    op: Option<Operation>,
    operand2: Option<Operand>,
}

#[derive(Debug, Clone)]
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
    resolve(&input, "root")
}

fn resolve(input: &HashMap<String, Monkey>, key: &str) -> i64 {
    if let Some(v) = input.get(key) {
        match (&v.operand1, &v.op, &v.operand2) {
            (Operand::Number(v), None, _) => *v,
            (Operand::Reference(op1), Some(op), Some(Operand::Reference(op2))) => match op {
                Operation::Divide => resolve(input, op1) / resolve(input, op2),
                Operation::Multiply => resolve(input, op1) * resolve(input, op2),
                Operation::Add => resolve(input, op1) + resolve(input, op2),
                Operation::Subtract => resolve(input, op1) - resolve(input, op2),
            },

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
