use day_11::part2;

fn main() {
    let file = include_str!("../input1.txt");
    let result = part2::process(file, 1000000);
    dbg!(result);
}
