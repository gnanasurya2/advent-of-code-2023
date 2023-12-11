use std::{cmp, collections::HashSet};

use itertools::Itertools;

pub fn process(input: &str, multiplier: usize) -> usize {
    let mut row_hash_set: HashSet<usize> = HashSet::new();
    let mut col_hash_set: HashSet<usize> = HashSet::new();
    let mut galaxy: Vec<(usize, usize)> = vec![];
    let game_input = input
        .lines()
        .enumerate()
        .map(|(r_index, line)| {
            line.chars()
                .enumerate()
                .map(|(c_index, ele)| {
                    if ele == '#' {
                        galaxy.push((r_index, c_index));
                        row_hash_set.insert(r_index);
                        col_hash_set.insert(c_index);
                        ele
                    } else {
                        ele
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut galaxy_less_row: Vec<usize> = vec![];
    let mut galaxy_less_column: Vec<usize> = vec![];

    for row in 0..game_input.len() {
        if !row_hash_set.contains(&row) {
            galaxy_less_row.push(row)
        }
    }

    for col in 0..game_input[0].len() {
        if !col_hash_set.contains(&col) {
            galaxy_less_column.push(col)
        }
    }

    galaxy
        .into_iter()
        .tuple_combinations::<((usize, usize), (usize, usize))>()
        .fold(0, |acc, ele| {
            let mut number_of_extra_rows: usize = 0;
            let mut number_of_extra_columns: usize = 0;
            let min_row = cmp::min(ele.0 .0, ele.1 .0);
            let max_row = cmp::max(ele.0 .0, ele.1 .0);
            let min_col = cmp::min(ele.0 .1, ele.1 .1);
            let max_col = cmp::max(ele.0 .1, ele.1 .1);

            for row in &galaxy_less_row {
                if *row > min_row && *row < max_row {
                    number_of_extra_rows += 1;
                }
            }

            for col in &galaxy_less_column {
                if *col > min_col && *col < max_col {
                    number_of_extra_columns += 1
                }
            }

            acc + max_row - min_row + max_col - min_col
                + (number_of_extra_rows + number_of_extra_columns) * (multiplier - 1)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input, 10);
        assert_eq!(result, 1030);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1_test.txt");
        let result = process(input, 100);
        assert_eq!(result, 8410);
    }
    #[test]
    fn it_works3() {
        let input = include_str!("./input1.txt");
        let result = process(input, 1000000);
        assert_eq!(result, 692506533832);
    }
}
