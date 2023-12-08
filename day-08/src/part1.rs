use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, val) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alpha1, tag(", "), alpha1),
            char(')'),
        ),
    )(input)?;

    Ok((input, val))
}
fn parse_input(input: &str) -> IResult<&str, (&str, Vec<(&str, (&str, &str))>)> {
    let (input, values) =
        separated_pair(alpha1, tag("\r\n\r\n"), separated_list1(tag("\r\n"), line))(input)?;

    Ok((input, values))
}
pub fn process(input: &str) -> u32 {
    let (_, (instructions, network)) = parse_input(input).unwrap();
    let instructions: Vec<char> = instructions.chars().collect();

    let network_map: BTreeMap<&str, (&str, &str)> = network.clone().into_iter().collect();
    let mut after_iteration_map: BTreeMap<&str, &str> = BTreeMap::new();

    let mut current_value: &str;
    let mut number_of_steps: u32;

    for val in &network {
        current_value = val.0;

        for ch in &instructions {
            if ch == &'L' {
                current_value = network_map.get(current_value).unwrap().0;
            } else {
                current_value = network_map.get(current_value).unwrap().1;
            }
        }
        after_iteration_map.insert(val.0, current_value);
    }

    number_of_steps = 0;
    current_value = "AAA";
    loop {
        let new_value = after_iteration_map.get(current_value).unwrap();
        if current_value == "ZZZ" {
            return number_of_steps;
        }

        current_value = new_value;
        number_of_steps += instructions.len() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 14681);
    }
}
