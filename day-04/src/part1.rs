pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
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
            let current_numbers: usize = line_iterator
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|num| {
                    winning_numbers.contains(&num.trim().parse::<i32>().unwrap_or_default())
                })
                .collect::<Vec<&str>>()
                .len();

            if current_numbers != 0 {
                usize::pow(2, current_numbers as u32 - 1)
            } else {
                0
            }
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 13);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 18653);
    }
}
