use day_06::part2_opt;

fn main() {
    let file = include_str!("../input1_test.txt");
    let result = part2_opt::process(file);
    dbg!(result);
}
