use day_16::part1;

fn main() {
    let file = include_str!("../input1.txt");
    let result = part1::process(file);
    dbg!(result);
}
