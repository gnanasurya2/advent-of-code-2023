#[derive(Debug)]
struct MachineNumber {
    value: u32,
    start: usize,
    end: usize,
}

fn find_adjacent_numbers(
    numbers: &Vec<Vec<MachineNumber>>,
    i: usize,
    j: usize,
    row_length: usize,
) -> Vec<u32> {
    let mut adjacent_numbers: Vec<u32> = numbers[i]
        .iter()
        .filter_map(|machine_number| {
            match machine_number.end == j - 1 || machine_number.start == j + 1 {
                true => Some(machine_number.value),
                false => None,
            }
        })
        .collect();

    if i as isize - 1 >= 0 {
        adjacent_numbers.extend(
            numbers[i - 1]
                .iter()
                .filter_map(|machine_number| {
                    match machine_number.start <= j + 1 && machine_number.end >= j - 1 {
                        true => Some(machine_number.value),
                        false => None,
                    }
                })
                .collect::<Vec<u32>>(),
        )
    }

    if i + 1 < row_length {
        adjacent_numbers.extend(
            numbers[i + 1]
                .iter()
                .filter_map(|machine_number| {
                    match machine_number.start <= j + 1 && machine_number.end >= j - 1 {
                        true => Some(machine_number.value),
                        false => None,
                    }
                })
                .collect::<Vec<u32>>(),
        )
    }
    adjacent_numbers
}
pub fn process(input: &str) -> u32 {
    let chars = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut is_part_of_number = false;
    let mut numbers: Vec<Vec<MachineNumber>> = vec![];
    let mut gear_ratio = 0;
    for i in 0..chars.len() {
        is_part_of_number = false;
        let mut start_index = 0;
        let mut number = 0;
        let mut row_numbers: Vec<MachineNumber> = vec![];
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
                    row_numbers.push(MachineNumber {
                        value: number,
                        start: start_index,
                        end: j - 1,
                    })
                }
            }
        }
        if is_part_of_number {
            row_numbers.push(MachineNumber {
                value: number,
                start: start_index,
                end: chars[i].len() - 1,
            })
        }
        numbers.push(row_numbers);
    }
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == '*' {
                let gear_numbers = find_adjacent_numbers(&numbers, i, j, chars.len());
                if gear_numbers.len() == 2 {
                    gear_ratio += gear_numbers[0] * gear_numbers[1];
                }
            }
        }
    }
    gear_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 467835);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 82824352);
    }
}
