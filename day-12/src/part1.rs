use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug)]
struct Row<'a> {
    wild_cards_count: u32,
    line: &'a str,
    constraints: Vec<u32>,
}

impl<'a> Row<'a> {
    fn permutations(&self) -> Vec<String> {
        let options: Vec<String> = repeat_n(["#", "."].into_iter(), self.wild_cards_count as usize)
            .multi_cartesian_product()
            .map(|c| c.join(""))
            .collect();
        options
    }

    fn check_permutation(&self, option: &str) -> bool {
        let mut option_iter = option.chars();

        let generated_constraints = self
            .line
            .chars()
            .map(|ch| match ch {
                '?' => option_iter.next().expect("should always have a value"),
                v => v,
            })
            .group_by(|ch| ch == &'#')
            .into_iter()
            .filter_map(|(is_spring, arr)| match is_spring {
                true => Some(arr.into_iter().count() as u32),
                false => None,
            })
            .collect::<Vec<u32>>();
        &generated_constraints[..] == &self.constraints[..]
    }

    fn possible_combinations_solution(&self) -> u32 {
        self.permutations()
            .iter()
            .filter(|option| self.check_permutation(option))
            .count() as u32
    }
}

fn parse_line(input: &str) -> IResult<&str, Row> {
    let (input, (line, values)) = separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;
    let wild_card_count = line.chars().filter(|c| c == &'?').count() as u32;
    Ok((
        input,
        Row {
            wild_cards_count: wild_card_count,
            line,
            constraints: values,
        },
    ))
}

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (_, row) = parse_line(line).expect("should have parsed");
            row.possible_combinations_solution()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 21);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 14681);
    }
}
