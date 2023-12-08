use std::collections::{BTreeMap, HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, val) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            char(')'),
        ),
    )(input)?;

    Ok((input, val))
}
fn parse_input(input: &str) -> IResult<&str, (&str, Vec<(&str, (&str, &str))>)> {
    let (input, values) = separated_pair(
        alphanumeric1,
        tag("\r\n\r\n"),
        separated_list1(tag("\r\n"), line),
    )(input)?;

    Ok((input, values))
}

fn gcd(num1: i64, num2: i64) -> i64 {
    if num2 == 0 {
        num1
    } else {
        gcd(num2, num1 % num2)
    }
}

fn lcm_of_array(arr: &Vec<i64>) -> i64 {
    let mut lcm = arr[0];

    for i in 1..arr.len() {
        let num1 = lcm;
        let num2 = arr[i];
        let gcd_val = gcd(num1, num2);
        lcm = (lcm * arr[i]) / gcd_val;
    }

    lcm
}

pub fn process(input: &str) -> i64 {
    let (_, (instructions, network)) = parse_input(input).unwrap();
    let instructions: Vec<char> = instructions.chars().collect();

    let mut network_map: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    let mut node_ending_with_a: Vec<&str> = vec![];

    network.iter().for_each(|node| {
        if node.0.ends_with("A") {
            node_ending_with_a.push(node.0);
        }
        network_map.insert(node.0, node.1);
    });

    let mut after_iteration_map = BTreeMap::new();
    for val in &network {
        let mut current_value = val.0;

        for ch in &instructions {
            if ch == &'L' {
                current_value = network_map.get(current_value).unwrap().0;
            } else {
                current_value = network_map.get(current_value).unwrap().1;
            }
        }

        after_iteration_map.insert(val.0, current_value);
    }

    let mut ends_with_z: Vec<i64> = vec![];
    for i in node_ending_with_a {
        let mut is_visited = HashMap::new();
        let mut number_of_iterations = 0;
        let mut current_value = i;

        loop {
            if is_visited.contains_key(current_value) {
                break;
            }
            is_visited.insert(current_value, true);
            number_of_iterations += 1;
            current_value = after_iteration_map.get(current_value).unwrap();

            if current_value.ends_with('Z') {
                ends_with_z.push(number_of_iterations);
            }
        }
    }

    lcm_of_array(&ends_with_z) * instructions.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 12);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 14321394058031);
    }
}
