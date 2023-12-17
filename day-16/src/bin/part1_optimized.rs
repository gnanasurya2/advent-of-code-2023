use day_16::part1_optimized;

fn main() {
    let file = include_str!("../input1.txt");
    let result = part1_optimized::process(file);
    dbg!(result);
}
