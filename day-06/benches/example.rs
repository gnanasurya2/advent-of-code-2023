use day_06;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_06::part1::process(black_box(input))
}
#[divan::bench]
fn part1_opt() -> u32 {
    let input = include_str!("../src/input1.txt");
    day_06::part1_opt::process(black_box(input))
}

#[divan::bench]
fn part2() -> u64 {
    let input = include_str!("../src/input1.txt");
    day_06::part2::process(black_box(input))
}

#[divan::bench]
fn part2_opt() -> u64 {
    let input = include_str!("../src/input1.txt");
    day_06::part2_opt::process(black_box(input))
}
