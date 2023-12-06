fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .filter_map(|val| match val != "" {
            true => Some(val.parse::<u32>().unwrap()),
            false => None,
        })
        .collect()
}
pub fn process(input: &str) -> u32 {
    let mut line_iterator = input.lines();
    let time_values: Vec<u32> = parse_numbers(line_iterator.next().unwrap());

    let distance_values: Vec<u32> = parse_numbers(line_iterator.next().unwrap());
    let mut ans = 1;
    for i in 0..time_values.len() {
        let half_value = time_values[i] / 2;
        let mut high_score_values = 0;
        for j in (0..=half_value).rev() {
            if j * (time_values[i] - j) > distance_values[i] {
                high_score_values += 1;
            } else {
                break;
            }
        }
        ans *= (high_score_values * 2) - if time_values[i] % 2 == 0 { 1 } else { 0 }
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
        assert_eq!(result, 13);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 18653);
    }
}
