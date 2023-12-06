fn parse_numbers(input: &str) -> u64 {
    input
        .split(":")
        .last()
        .unwrap()
        .trim()
        .chars()
        .filter(|x| *x != ' ')
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
pub fn process(input: &str) -> u64 {
    let mut line_iterator = input.lines();
    let time_values = parse_numbers(line_iterator.next().unwrap());

    let distance_values = parse_numbers(line_iterator.next().unwrap());
    let mut ans = 1;
    let half_value = time_values / 2;
    for j in 0..=half_value {
        if j * (time_values - j) > distance_values {
            ans = time_values - (j - 1) * 2 - 1;
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 71503);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 38220708);
    }
}
