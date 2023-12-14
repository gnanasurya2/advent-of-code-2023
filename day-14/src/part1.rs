pub fn process(input: &str) -> usize {
    let mut column_iterator = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    std::iter::from_fn(move || {
        let mut items = vec![];

        for iters in &mut column_iterator {
            match iters.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }
        Some(items)
    })
    .map(|col| {
        let mut location_to_fill = 0;
        let mut sum = 0;
        for i in 0..col.len() {
            match col[i] {
                'O' => {
                    sum += col.len() - location_to_fill;
                    location_to_fill += 1;
                }
                '#' => {
                    location_to_fill = i + 1;
                }
                _ => (),
            }
        }
        sum
    })
    .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 136);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 110565);
    }
}
