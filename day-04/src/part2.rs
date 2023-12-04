use std::cmp;

pub fn process(input: &str) -> u32 {
    let lines = input.lines();
    let mut line_count = 0;
    let numbers: Vec<usize> = lines
        .map(|line| {
            line_count += 1;
            let mut line_iterator = line.split("|");
            let winning_numbers: Vec<i32> = line_iterator
                .next()
                .unwrap()
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .map(|num| num.trim().parse::<i32>().unwrap_or(-1))
                .collect();
            line_iterator
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|num| {
                    winning_numbers.contains(&num.trim().parse::<i32>().unwrap_or_default())
                })
                .collect::<Vec<&str>>()
                .len()
        })
        .collect();
    let mut number_of_cards = vec![1; line_count];

    for i in 0..number_of_cards.len() {
        if numbers[i] != 0 {
            for j in i + 1..cmp::min(i + 1 + numbers[i], line_count) {
                number_of_cards[j] += number_of_cards[i];
            }
        }
    }

    number_of_cards.iter().sum()
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
        assert_eq!(result, 5921508);
    }
}
