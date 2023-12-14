use std::collections::HashMap;

fn perform_east_tilt(input: &mut Vec<Vec<char>>, size: usize) -> &mut Vec<Vec<char>> {
    for i in 0..size {
        let mut location_to_fill = size - 1;
        for j in (0..size).rev() {
            match input[i][j] {
                'O' => {
                    input[i][j] = '.';
                    input[i][location_to_fill] = 'O';
                    if location_to_fill != 0 {
                        location_to_fill -= 1;
                    }
                }
                '#' => {
                    if j != 0 {
                        location_to_fill = j - 1;
                    }
                }
                _ => (),
            }
        }
    }
    input
}
fn perform_south_tilt(input: &mut Vec<Vec<char>>, size: usize) -> &mut Vec<Vec<char>> {
    for j in 0..size {
        let mut location_to_fill = size - 1;
        for i in (0..size).rev() {
            match input[i][j] {
                'O' => {
                    input[i][j] = '.';
                    input[location_to_fill][j] = 'O';
                    if location_to_fill != 0 {
                        location_to_fill -= 1;
                    }
                }
                '#' => {
                    if i != 0 {
                        location_to_fill = i - 1;
                    }
                }
                _ => (),
            }
        }
    }
    input
}
fn perform_west_tilt(input: &mut Vec<Vec<char>>, size: usize) -> &mut Vec<Vec<char>> {
    for i in 0..size {
        let mut location_to_fill = 0;
        for j in 0..size {
            match input[i][j] {
                'O' => {
                    input[i][j] = '.';
                    input[i][location_to_fill] = 'O';
                    location_to_fill += 1;
                }
                '#' => {
                    location_to_fill = j + 1;
                }
                _ => (),
            }
        }
    }
    input
}
fn perform_north_tilt(input: &mut Vec<Vec<char>>, size: usize) -> &mut Vec<Vec<char>> {
    for j in 0..size {
        let mut location_to_fill = 0;
        for i in 0..size {
            match input[i][j] {
                'O' => {
                    input[i][j] = '.';
                    input[location_to_fill][j] = 'O';
                    location_to_fill += 1;
                }
                '#' => {
                    location_to_fill = i + 1;
                }
                _ => (),
            }
        }
    }
    input
}

pub fn process(input: &str) -> usize {
    let size = (input.len() as f64).sqrt() as usize;
    let mut column_iterator = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut map: HashMap<Vec<Vec<char>>, (Vec<Vec<char>>, i32)> = HashMap::new();
    let mut number_of_iteration = 1000000000;
    let mut current_iteration = 0;
    let mut remaining_iterations = 0;
    while number_of_iteration > 0 {
        let cloned_value = column_iterator.clone();

        let hash_value = map.get(&cloned_value);
        if let Some(hash_value) = hash_value {
            remaining_iterations = number_of_iteration % (current_iteration - hash_value.1);
            break;
        }

        perform_north_tilt(&mut column_iterator, size);
        perform_west_tilt(&mut column_iterator, size);
        perform_south_tilt(&mut column_iterator, size);
        perform_east_tilt(&mut column_iterator, size);
        map.insert(cloned_value, (column_iterator.clone(), current_iteration));
        current_iteration += 1;

        number_of_iteration -= 1;
    }

    while remaining_iterations > 0 {
        remaining_iterations -= 1;
        perform_north_tilt(&mut column_iterator, size);
        perform_west_tilt(&mut column_iterator, size);
        perform_south_tilt(&mut column_iterator, size);
        perform_east_tilt(&mut column_iterator, size);
    }
    let mut sum = 0;
    for i in 0..size {
        for j in 0..size {
            match column_iterator[i][j] {
                'O' => {
                    sum += size - i;
                }
                _ => {}
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 64);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 89845);
    }
}
