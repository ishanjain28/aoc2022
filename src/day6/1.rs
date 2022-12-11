#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn parse(input: &[u8]) -> &[u8] {
    input.trim_ascii()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution(output);

        println!("{}", score);
    }
}

fn solution(input: &[u8]) -> usize {
    let mut buffer = [0; 26];

    for &c in input.iter().take(4) {
        buffer[(c - b'a') as usize] += 1;
    }

    if buffer.iter().all(|&c| c == 0 || c == 1) {
        return 0;
    }

    for i in 4..input.len() {
        let c = input[i];
        buffer[(c - b'a') as usize] += 1;
        let last = input[i - 4];
        buffer[(last - b'a') as usize] -= 1;

        if buffer.iter().all(|&c| c == 0 || c == 1) {
            return i + 1;
        }
    }

    0
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution(input);
        test::black_box(result);
    })
}

#[test]
fn tests() {
    assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 5);
    assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 6);
    assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 10);
    assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 11);
}
