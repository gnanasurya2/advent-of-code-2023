use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::cmp;

fn split_number_from_str(nums: Vec<&str>) -> Vec<Vec<i64>> {
    nums.iter()
        .filter_map(|line| {
            let splitted: Vec<&str> = line.split(" ").collect();
            match splitted.len() == 3 {
                true => Some(
                    splitted
                        .clone()
                        .iter()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>(),
                ),
                false => None,
            }
        })
        .collect()
}

pub fn process(input: &str) -> i64 {
    let mut line_iterator = input.lines();
    let seeds: Vec<i64> = line_iterator
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    let mut mappings: Vec<Vec<Vec<i64>>> = vec![];
    let mut current_lines: Vec<&str> = vec![];
    for line in line_iterator.skip(1) {
        if line != "" {
            current_lines.push(line);
        } else {
            mappings.push(split_number_from_str(current_lines));
            current_lines = vec![];
        }
    }

    mappings.push(split_number_from_str(current_lines));
    let mut min_value = i64::MAX;
    let seeds: Vec<(i64, i64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    for seed_range in seeds {
        let max_value = seed_range.0 + seed_range.1;
        let current_value = (seed_range.0..max_value)
            .into_par_iter()
            .progress_count((max_value) as u64)
            .map(|seed| {
                let mut current_value = seed;
                for mapping in &mappings {
                    for i in 0..mapping.len() {
                        let max_value = mapping[i][2] + mapping[i][1];
                        if max_value > current_value && max_value - current_value <= mapping[i][2] {
                            current_value = mapping[i][0] + current_value - mapping[i][1];
                            break;
                        }
                    }
                }
                current_value
            })
            .min()
            .unwrap();
        min_value = cmp::min(min_value, current_value);
    }
    min_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 30);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 15880236);
    }
}
