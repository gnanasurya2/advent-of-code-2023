use day_11::part1;

fn main() {
    let file = include_str!("../input1_test.txt");
    let result = part1::process(file);
    dbg!(result);
}
