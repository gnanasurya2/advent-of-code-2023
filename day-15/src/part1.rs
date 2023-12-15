pub fn process(input: &str) -> u32 {
    input
        .split(",")
        .map(|hash_input| {
            hash_input.chars().fold(0u32, |mut acc, ch| {
                acc += ch as u32;
                acc *= 17;
                acc % 256
            })
        })
        .sum::<u32>()
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
