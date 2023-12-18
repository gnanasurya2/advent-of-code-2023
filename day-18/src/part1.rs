use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, alphanumeric1, char, hex_digit1, line_ending, space1},
        streaming::digit1,
    },
    combinator::{self, map_res},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use std::{cmp, str::FromStr};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Instruction<'a> {
    direction: Direction,
    length: i32,
    color: &'a str,
    position: (i32, i32),
}
fn parse_input(input: &str) -> IResult<&str, Instruction> {
    let (input, (direction, _, length, _, color, _)) = tuple((
        alt((
            combinator::map(char('U'), |_| Direction::Up),
            combinator::map(char('D'), |_| Direction::Down),
            combinator::map(char('L'), |_| Direction::Left),
            combinator::map(char('R'), |_| Direction::Right),
        )),
        space1,
        map_res(digit1, i32::from_str),
        tag(" (#"),
        alphanumeric1,
        tag(")"),
    ))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            length,
            color,
            position: (0, 0),
        },
    ))
}

fn valid_point(point: &(i32, i32), row_limit: i32, col_limit: i32) -> bool {
    point.0 >= 0 && point.0 <= col_limit && point.1 >= 0 && point.1 <= row_limit
}

fn is_inside(grid: &Vec<Vec<char>>, point: &(i32, i32)) -> bool {
    let mut num_of_walls = 0;
    for i in 0..point.1 as usize {
        if grid[point.0 as usize][i] == '#' {
            num_of_walls += 1;
        }
    }
    num_of_walls % 2 != 0
}

pub fn process(input: &str) -> u32 {
    let mut length = (0, 0);
    let mut height = (0, 0);
    let mut current = (0, 0);
    let instructions = input
        .lines()
        .map(|line| {
            let (_, mut instruction) = parse_input(line).expect("should have parsed");
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
            instruction.position = current;
            length = (cmp::min(length.0, current.0), cmp::max(length.1, current.0));
            height = (cmp::min(height.0, current.1), cmp::max(height.1, current.1));
            instruction
        })
        .collect::<Vec<Instruction>>();

    let mut current = (-length.0 as usize, -height.0 as usize);

    let length = length.1 - length.0;
    let height = height.1 - height.0;

    let mut grid = (0..=length)
        .map(|_| (0..=height).map(|_| '.').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    instructions.iter().for_each(|ins| match ins.direction {
        Direction::Up => {
            for _ in 0..ins.length {
                current.0 -= 1;
                grid[current.0][current.1] = '#';
            }
        }
        Direction::Down => {
            for _ in 0..ins.length {
                current.0 += 1;
                grid[current.0][current.1] = '#';
            }
        }
        Direction::Right => {
            for _ in 0..ins.length {
                current.1 += 1;
                grid[current.0][current.1] = '#';
            }
        }
        Direction::Left => {
            for _ in 0..ins.length {
                current.1 -= 1;
                grid[current.0][current.1] = '#';
            }
        }
    });
    for i in 0..=length as usize {
        for j in 0..=height as usize {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
    println!("");
    let current = (current.0 as i32, current.1 as i32);
    let binding = [
        (current.0 - 1, current.1 + 1),
        (current.0 - 1, current.1 - 1),
        (current.0 + 1, current.1 + 1),
        (current.0 + 1, current.1 - 1),
    ]
    .into_iter()
    .filter(|ele| valid_point(ele, height, length) && is_inside(&grid, ele))
    .collect::<Vec<(i32, i32)>>();

    let mut size = 0;
    let mut nodes = vec![(binding[0].0, binding[0].1)];

    while nodes.len() != 0 {
        let curr_node = nodes.pop().unwrap();

        [
            (curr_node.0 - 1, curr_node.1),
            (curr_node.0 + 1, curr_node.1),
            (curr_node.0, curr_node.1 - 1),
            (curr_node.0, curr_node.1 + 1),
        ]
        .into_iter()
        .for_each(|ele| {
            if valid_point(&ele, height, length) {
                // println!("curr: {:?} neigh: {:?}", curr_node, ele);
                match grid[ele.0 as usize][ele.1 as usize] {
                    '#' => {
                        grid[ele.0 as usize][ele.1 as usize] = '#';
                    }
                    '.' => {
                        grid[ele.0 as usize][ele.1 as usize] = '#';
                        nodes.push(ele);
                    }
                    _ => {}
                }
            }
        })
    }
    for i in 0..=length as usize {
        for j in 0..=height as usize {
            print!("{}", grid[i][j]);
            if grid[i][j] == '#' {
                size += 1;
            }
        }
        println!("");
    }
    // dbg!(length, height, current, binding,);
    (size) as u32
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
