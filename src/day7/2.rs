#![feature(test)]

use std::collections::HashMap;
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn parse(input: &'static str) -> impl Iterator<Item = &'static str> {
    let input = input.trim().lines();

    input
}

#[derive(Debug)]
enum Node {
    File(u32),
    Directory(Directory),
}

#[derive(Debug)]
struct Directory {
    children: HashMap<String, Node>,
    parent: *mut Node,
    size: u32,
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{}", score);
    }
}

fn solution(input: impl Iterator<Item = &'static str>) -> u32 {
    let mut tree = Node::Directory(Directory {
        children: HashMap::new(),
        parent: std::ptr::null_mut(),
        size: 0,
    });

    let mut current = &mut tree;

    for line in input {
        match &line[..4] {
            "$ ls" => continue,

            "dir " => {
                let dir = &line[4..];
                let cptr = current as *mut Node;

                let Node::Directory(Directory { children, ..}) = current else { unreachable!()};

                children.insert(
                    dir.to_string(),
                    Node::Directory(Directory {
                        children: HashMap::new(),
                        parent: cptr,
                        size: 0,
                    }),
                );
            }

            "$ cd" => match &line[5..6] {
                // we are supposed to match on .. but this is fine
                "." => {
                    let Node::Directory(Directory { parent , .. }) = *current  else {
                        unreachable!("current is not a directory");
                    };

                    current = unsafe { &mut *parent };
                }

                "/" => current = &mut tree,

                _ => {
                    let dir = &line[5..];
                    let Node::Directory(Directory { children, ..})  = current  else {
                            unreachable!("current is not a directory");
                        };

                    current = children.get_mut(dir).unwrap();
                }
            },

            _ => {
                let (size, name) = line.split_once(' ').unwrap();
                let fsize = size
                    .bytes()
                    .take_while(|&c| c != b' ')
                    .fold(0u32, |a, x| a * 10 + (x - b'0') as u32);

                let Node::Directory(v) = current else {unreachable!("not a directory") };

                v.children
                    .entry(name.to_string())
                    .or_insert(Node::File(fsize));
            }
        }
    }

    compute_dir_size(&mut tree);

    let total = 70000000;
    let required = 30000000;

    let Node::Directory(ref root) = tree else { unreachable!("not a directory") };

    let to_freeup = root.size + required - total;

    part2(&tree, to_freeup)
}

fn part2(node: &Node, to_freeup: u32) -> u32 {
    match node {
        Node::Directory(dir) if dir.size >= to_freeup => {
            let mut answer = dir.size;

            for v in dir.children.values() {
                answer = std::cmp::min(answer, part2(v, to_freeup));
            }

            answer
        }
        _ => std::u32::MAX,
    }
}

fn compute_dir_size(node: &mut Node) -> u32 {
    match node {
        &mut Node::File(v) => v,

        Node::Directory(dir) => {
            let mut sum = 0;

            for v in dir.children.values_mut() {
                sum += compute_dir_size(v);
            }

            dir.size = sum;
            sum
        }
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
