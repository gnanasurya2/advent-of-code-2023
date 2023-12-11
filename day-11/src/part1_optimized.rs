use std::{cmp, collections::HashSet};

use itertools::Itertools;

pub fn process(input: &str) -> u32 {
    let number_of_rows = 140;
    let number_of_cols = 140;
    let mut row_hash_set: HashSet<usize> = (0..number_of_rows).collect();
    let mut col_hash_set: HashSet<usize> = (0..number_of_cols).collect();
    let mut galaxy: Vec<(usize, usize)> = vec![];

    input.lines().enumerate().for_each(|(r_index, line)| {
        line.chars().enumerate().for_each(|(c_index, ele)| {
            if ele == '#' {
                galaxy.push((r_index, c_index));
                row_hash_set.remove(&r_index);
                col_hash_set.remove(&c_index);
            }
        })
    });

    let galaxy_less_row: Vec<&usize> = row_hash_set.iter().collect();
    let galaxy_less_column: Vec<&usize> = col_hash_set.iter().collect();

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
                if **row > min_row && **row < max_row {
                    number_of_extra_rows += 1;
                }
            }

            for col in &galaxy_less_column {
                if **col > min_col && **col < max_col {
                    number_of_extra_columns += 1;
                }
            }
            acc + max_row - min_row + max_col - min_col
                + number_of_extra_rows
                + number_of_extra_columns
        }) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 374);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 9918828);
    }
}
