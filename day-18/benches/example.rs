use day_18;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_18::part1::process(black_box(input))
}

#[divan::bench]
fn part2() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_18::part2::process(black_box(input))
}
