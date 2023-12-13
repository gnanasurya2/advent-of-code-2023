use day_13;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> usize {
    let input = include_str!("../src/input1.txt");
    day_13::part1::process(black_box(input))
}

#[divan::bench]
fn part2() -> usize {
    let input = include_str!("../src/input1.txt");
    day_13::part2::process(black_box(input))
}
