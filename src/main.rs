const INPUTS: [&str; 2] = [
    include_str!("../inputs/sample.txt"),
    include_str!("../inputs/input.txt"),
];

fn parse(input: &'static str) -> Vec<(Move, Outcome)> {
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
                "X" => Outcome::X,
                "Y" => Outcome::Y,
                "Z" => Outcome::Z,
                _ => unreachable!(),
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

#[derive(Clone, Copy)]
enum Outcome {
    X = 0,
    Y = 3,
    Z = 6,
}

fn calc_move(d1: Outcome, m1: Move) -> Move {
    match (d1, m1) {
        (Outcome::X, Move::Rock) => Move::Scissors,
        (Outcome::X, Move::Paper) => Move::Rock,
        (Outcome::X, Move::Scissors) => Move::Paper,
        (Outcome::Y, v) => v,
        (Outcome::Z, Move::Rock) => Move::Paper,
        (Outcome::Z, Move::Paper) => Move::Scissors,
        (Outcome::Z, Move::Scissors) => Move::Rock,
    }
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let mut score = 0;

        for (a, b) in output {
            score += b as i32;
            score += calc_move(b, a) as i32;
        }

        println!("{:?}", score);
    }
}
