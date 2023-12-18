use day_18::part2;

fn main() {
    let file = include_str!("../input1_test.txt");
    let result = part2::process(file);
    dbg!(result);
}
