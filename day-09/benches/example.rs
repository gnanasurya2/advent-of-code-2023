use day_09;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> i64 {
    let input = include_str!("../src/input1.txt");
    day_09::part1::process(black_box(input))
}

#[divan::bench]
fn part2() -> i64 {
    let input = include_str!("../src/input1.txt");
    day_09::part2::process(black_box(input))
}
