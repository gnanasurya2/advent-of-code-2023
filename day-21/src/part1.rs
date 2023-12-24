use std::{collections::HashSet, time::Instant};

pub fn process(input: &str) -> u32 {
    let start = Instant::now();
    // let mut starting_index = (0, 0);
    let garden_map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| match ch {
                    'S' => {
                        // starting_index = (i, j);
                        // println!("{:?}", starting_index);
                        true
                    }
                    '.' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut number_of_iterations = 64;

    let mut current_nodes = vec![(65, 65)];
    // println!("{:?}", current_nodes);
    while number_of_iterations != 0 {
        number_of_iterations -= 1;
        let mut visited_nodes: HashSet<(isize, isize)> = HashSet::new();
        let mut next_nodes: Vec<(isize, isize)> = vec![];
        for node in &current_nodes {
            [
                (node.0 - 1, node.1),
                (node.0 + 1, node.1),
                (node.0, node.1 - 1),
                (node.0, node.1 + 1),
            ]
            .iter()
            .filter(|ele| {
                ele.0 >= 0
                    && ele.0 < garden_map.len() as isize
                    && ele.1 >= 0
                    && ele.1 < garden_map[0].len() as isize
            })
            .for_each(|curr| {
                if garden_map[curr.0 as usize][curr.1 as usize] && !visited_nodes.contains(curr) {
                    visited_nodes.insert(*curr);
                    next_nodes.push(*curr);
                }
            });
        }

        current_nodes = next_nodes;
        // println!("{}-{}", 64 - number_of_iterations, current_nodes.len());
    }
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    current_nodes.len() as u32
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
