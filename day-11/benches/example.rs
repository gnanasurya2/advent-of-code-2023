use day_11;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_11::part1::process(black_box(input))
}

#[divan::bench]
fn part1_opt() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_11::part1_optimized::process(black_box(input))
}

#[divan::bench]
fn part2() -> usize {
    let input = include_str!("../src/input1.txt");
    day_11::part2::process(black_box(input), 10)
}
