use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alphanumeric1, char, space1},
        streaming::digit1,
    },
    combinator::{self, map_res},
    sequence::tuple,
    IResult,
};

use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: i64,
}
fn parse_input(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, _, _, _, color, _)) = tuple((
        alt((
            combinator::map(char('U'), |_| Direction::Up),
            combinator::map(char('D'), |_| Direction::Down),
            combinator::map(char('L'), |_| Direction::Left),
            combinator::map(char('R'), |_| Direction::Right),
        )),
        space1,
        map_res(digit1, i64::from_str),
        tag(" (#"),
        alphanumeric1,
        tag(")"),
    ))(input)?;

    Ok((
        input,
        Instruction {
            direction: match &color[5..] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                _ => Direction::Up,
            },
            length: i64::from_str_radix(&color[..5], 16).unwrap(),
        },
    ))
}

pub fn process(input: &str) -> i64 {
    let mut current = (0, 0);
    let mut boundary = 0;
    let instructions = input
        .lines()
        .map(|line| {
            let (_, instruction) = parse_input(line).expect("should have parsed");
            match instruction.direction {
                Direction::Up => {
                    current.0 -= instruction.length;
                }
                Direction::Down => {
                    current.0 += instruction.length;
                }
                Direction::Right => {
                    current.1 += instruction.length;
                }
                Direction::Left => {
                    current.1 -= instruction.length;
                }
            }
            boundary += instruction.length;

            current
        })
        .collect::<Vec<(i64, i64)>>();

    let area: i64 = instructions
        .iter()
        .zip(instructions.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();

    area.abs() / 2 + boundary / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 1320);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 498538);
    }
}
