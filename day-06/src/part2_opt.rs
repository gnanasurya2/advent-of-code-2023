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

fn binary_search(low: u64, high: u64, time_value: u64, distance_value: u64) -> u64 {
    let temp = (low + high) / 2;
    if temp * (time_value - temp) > distance_value {
        if (temp - 1) * (time_value - temp + 1) <= distance_value {
            return temp;
        } else {
            return binary_search(0, temp, time_value, distance_value);
        }
    } else {
        return binary_search(temp, high, time_value, distance_value);
    }
}

pub fn process(input: &str) -> u64 {
    let mut line_iterator = input.lines();
    let time_values = parse_numbers(line_iterator.next().unwrap());

    let distance_values = parse_numbers(line_iterator.next().unwrap());

    (time_values / 2 - binary_search(0, time_values, time_values, distance_values) + 1) * 2
        - if time_values % 2 != 0 { 0 } else { 1 }
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
