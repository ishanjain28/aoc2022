const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> Vec<(Move, Move)> {
    input
        .trim()
        .lines()
        .map(|set| {
            let set = set.trim();
            let (a, b) = set.split_at(1);

            let x = match a.trim() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => unreachable!(),
            };
            let y = match b.trim() {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                v => {
                    println!("{:?}", v);

                    unreachable!();
                }
            };
            (x, y)
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn calc_score(m1: Move, m2: Move) -> i32 {
    match (m1, m2) {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 0,
        (Move::Rock, Move::Scissors) => 6,
        (Move::Paper, Move::Rock) => 6,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Scissors, Move::Rock) => 0,
        (Move::Scissors, Move::Paper) => 6,
        (Move::Scissors, Move::Scissors) => 3,
    }
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let mut score = 0;

        for (a, b) in output {
            score += b as i32;
            score += calc_score(b, a);
        }

        println!("{:?}", score);
    }
}
