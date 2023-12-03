fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn is_symbol(ch: &char) -> bool {
    !ch.is_numeric() && *ch != '.'
}

fn is_machine_part(
    chars: &Vec<Vec<char>>,
    start_index: usize,
    end_index: usize,
    row: usize,
    row_length: usize,
    column_length: usize,
) -> bool {
    if start_index as isize - 1 >= 0 && is_symbol(&chars[row][start_index - 1]) {
        true
    } else if end_index + 1 < row_length && is_symbol(&chars[row][end_index + 1]) {
        true
    } else {
        let mut is_valid = false;
        if row as isize - 1 >= 0 {
            for i in start_index as isize - 1..end_index as isize + 2 {
                if i as isize >= 0
                    && i < row_length as isize
                    && is_symbol(&chars[row - 1][i as usize])
                {
                    is_valid = true;
                    break;
                }
            }
        }

        if row + 1 < column_length {
            for i in start_index as isize - 1..end_index as isize + 2 {
                if i as isize >= 0
                    && i < row_length as isize
                    && is_symbol(&chars[row + 1][i as usize])
                {
                    is_valid = true;
                    break;
                }
            }
        }
        is_valid
    }
}
fn part1(input: &str) -> u32 {
    let chars = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut is_part_of_number = false;
    let mut sum = 0;
    for i in 0..chars.len() {
        is_part_of_number = false;
        let mut start_index = 0;
        let mut number = 0;
        for j in 0..chars[i].len() {
            if !is_part_of_number && chars[i][j].is_numeric() {
                is_part_of_number = true;
                start_index = j;
                number = chars[i][j].to_digit(10).unwrap();
            } else if is_part_of_number {
                if chars[i][j].is_numeric() {
                    number = number * 10 + chars[i][j].to_digit(10).unwrap();
                } else {
                    is_part_of_number = false;
                    if is_machine_part(&chars, start_index, j - 1, i, chars[i].len(), chars.len()) {
                        sum += number;
                    }
                }
            }
        }
        if is_part_of_number {
            if is_machine_part(
                &chars,
                start_index,
                chars[i].len() - 1,
                i,
                chars[i].len(),
                chars.len(),
            ) {
                sum += number;
            }
        }
    }
    // dbg!(chars);
    sum
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, 556057);
    }
}
