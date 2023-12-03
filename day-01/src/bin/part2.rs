use day_01::part2;

fn main() {
    let file = include_str!("../input1.txt");
    let result = part2::process(file);
    dbg!(result);
}
