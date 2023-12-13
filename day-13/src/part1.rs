use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Vec<&str>>>> {
    let (input, response) = separated_list1(
        pair(line_ending, line_ending),
        separated_list1(
            line_ending,
            preceded(space0, many1(alt((tag("#"), tag("."))))),
        ),
    )(input)?;
    Ok((input, response))
}
fn is_perfect_reflection(input: &Vec<Vec<&str>>, start: usize, len: usize) -> bool {
    let mut first_pointer: isize = start as isize;
    let mut end_pointer = start + 1;

    while first_pointer >= 0 && end_pointer < len {
        if input[first_pointer as usize] != input[end_pointer] {
            return false;
        }
        first_pointer -= 1;
        end_pointer += 1
    }
    true
}

pub fn process(input: &str) -> usize {
    let (_, response) = parse_input(input).expect("should parse everytime");
    response
        .iter()
        .map(|game: &Vec<Vec<&str>>| {
            for i in 0..game.len() - 1 {
                if is_perfect_reflection(game, i, game.len()) {
                    return (i + 1) * 100;
                }
            }
            let row_len: usize = game.len();
            let col_len = game[0].len();
            let updated_game = (0..col_len)
                .map(|j| (0..row_len).map(|i| game[i][j]).collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();

            for i in 0..updated_game.len() - 1 {
                if is_perfect_reflection(&updated_game, i, updated_game.len()) {
                    return i + 1;
                }
            }
            return 0;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 405);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 40006);
    }
}
