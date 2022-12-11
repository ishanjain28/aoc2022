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
        let score = solution::<14>(output);

        println!("{}", score);
    }
}

fn solution<const N: usize>(input: &[u8]) -> usize {
    let mut buffer = 0u32;
    for idx in 0..input.len() {
        // Set the bit when we enter
        buffer ^= 1 << (input[idx] - b'a');

        // Reset it when we leave
        if idx >= N {
            buffer ^= 1 << (input[idx - N] - b'a');
        }

        if buffer.count_ones() as usize == N {
            return idx + 1;
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
