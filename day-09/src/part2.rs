use nom::{
    character::complete::{i64, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn line(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, separated_list1(space1, i64))(input)
}

pub fn process(input: &str) -> i64 {
    let (_, mut histories) = line(input).unwrap();

    histories
        .iter_mut()
        .map(|history| {
            history.reverse();
            let mut length = history.len();
            let mut number_of_iterations = 0;
            loop {
                let mut is_all_zero = true;
                number_of_iterations += 1;
                length -= 1;

                for i in 0..length {
                    let new_value = history[i + 1] - history[i];

                    if new_value != 0 {
                        is_all_zero = false
                    }
                    history[i] = new_value
                }

                if is_all_zero {
                    let mut extrapolated_value: i64 = 0;
                    for i in (history.len() - number_of_iterations - 1)..history.len() {
                        extrapolated_value += history[i];
                    }
                    return extrapolated_value;
                }
            }
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 942);
    }
}
