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
    let mut buffer = 0u32;

    for i in 0..input.len() {
        buffer ^= 1 << (input[i] - b'a');

        if i >= 4 {
            buffer ^= 1 << (input[i - 4] - b'a');
        }

        if buffer.count_ones() == 4 {
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
