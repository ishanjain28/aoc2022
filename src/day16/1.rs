#![feature(test)]

use std::collections::HashMap;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Node {
    flow: usize,
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
                    'A'..='Z' => node.push(c),
                    '0'..='9' => num = num * 10 + (c as u8 - b'0') as usize,
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

const MAX_TIME: usize = 30;

fn solution(input: HashMap<String, Node>) -> usize {
    let n = input.len();
    let mut grid = vec![vec![1000; n]; n];

    for v in input.values() {
        grid[v.idx][v.idx] = 0;

        for node in v.leads_to.iter() {
            let idx = input.get(node).unwrap().idx;

            grid[v.idx][idx] = 1;
            grid[idx][v.idx] = 1;
        }
    }

    // Floyd warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                grid[i][j] = std::cmp::min(grid[i][k] + grid[k][j], grid[i][j]);
            }
        }
    }

    let posflowvalves: Vec<String> = input
        .iter()
        .filter(|(_, b)| b.flow > 0)
        .map(|(a, _)| a)
        .cloned()
        .collect();

    let mut memo = HashMap::new();

    dfs(&grid, &input, &posflowvalves, "AA", 0, 0, &mut memo)
}

fn dfs<'a>(
    grid: &[Vec<usize>],
    input: &HashMap<String, Node>,
    posflowvalves: &'a [String],
    node: &'a str,
    ctime: usize,
    seen: usize,
    memo: &mut HashMap<(&'a str, usize, usize), usize>,
) -> usize {
    let mut answer = 0;
    let inode = input.get(node).unwrap();

    if let Some(x) = memo.get(&(node, ctime, seen)) {
        return *x;
    }

    for (neighbour, nnode) in posflowvalves
        .iter()
        .filter_map(|c| input.get(c).map(|x| (c, x)))
    {
        // only visit a  node if it's not seen
        if seen & (1 << nnode.idx) == 0 {
            let ctime = ctime + grid[inode.idx][nnode.idx] + 1;

            if ctime <= MAX_TIME {
                answer = std::cmp::max(
                    answer,
                    (MAX_TIME - ctime) * nnode.flow
                        + dfs(
                            grid,
                            input,
                            posflowvalves,
                            neighbour,
                            ctime,
                            seen | (1 << nnode.idx),
                            memo,
                        ),
                );
            }
        }
    }

    memo.insert((node, ctime, seen), answer);
    answer
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
