#![feature(byte_slice_trim_ascii)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("../inputs/sample.txt"),
    include_bytes!("../inputs/input.txt"),
];

fn parse(input: &[u8]) -> &[u8] {
    input.trim_ascii()
}

fn main() {
    for input in INPUTS.iter() {
        let output = parse(input);
        let score = solution::<14>(output);

        println!("{}", score);
    }
}

fn solution<const N: usize>(input: &[u8]) -> usize {
    for idx in 0..input.len() - N {
        let mut buffer = 0u32;

        for &c in &input[idx..(idx + N)] {
            buffer |= 1 << (c - b'a') as usize;
        }

        if buffer.count_ones() as usize == N {
            return idx + N;
        }
    }

    0
}

#[bench]
fn solution_bench(b: &mut test::Bencher) {
    b.iter(|| {
        let input = parse(INPUTS[1]);
        let result = solution::<14>(input);
        test::black_box(result);
    })
}

#[test]
fn tests() {
    assert_eq!(
        solution::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()),
        23
    );
    assert_eq!(
        solution::<14>("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()),
        23
    );
    assert_eq!(
        solution::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()),
        29
    );
    assert_eq!(
        solution::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()),
        26
    );
}
