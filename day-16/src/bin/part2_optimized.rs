use day_16::part2_optimized;

fn main() {
    let file = include_str!("../input1.txt");
    let result = part2_optimized::process(file);
    dbg!(result);
}
