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

fn binary_search(low: u32, high: u32, time_value: u32, distance_value: u32) -> u32 {
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

pub fn process(input: &str) -> u32 {
    let mut line_iterator = input.lines();
    let time_values: Vec<u32> = parse_numbers(line_iterator.next().unwrap());

    let distance_values: Vec<u32> = parse_numbers(line_iterator.next().unwrap());
    let mut ans = 1;
    for i in 0..time_values.len() {
        ans *= (time_values[i] / 2
            - binary_search(0, time_values[i], time_values[i], distance_values[i])
            + 1)
            * 2
            - if time_values[i] % 2 != 0 { 0 } else { 1 }
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
        assert_eq!(result, 288);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 741000);
    }
}
