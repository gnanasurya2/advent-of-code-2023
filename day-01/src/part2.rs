pub fn process(input: &str) -> u32 {
    let mut final_sum = 0;
    for calibration_value in input.split("\r\n") {
        let mut replaced_value: Vec<u32> = vec![];
        for i in 0..calibration_value.len() {
            let sliced = &calibration_value[i..];
            if sliced.starts_with("one") {
                replaced_value.push(1);
            } else if sliced.starts_with("two") {
                replaced_value.push(2);
            } else if sliced.starts_with("three") {
                replaced_value.push(3);
            } else if sliced.starts_with("four") {
                replaced_value.push(4);
            } else if sliced.starts_with("five") {
                replaced_value.push(5);
            } else if sliced.starts_with("six") {
                replaced_value.push(6);
            } else if sliced.starts_with("seven") {
                replaced_value.push(7);
            } else if sliced.starts_with("eight") {
                replaced_value.push(8);
            } else if sliced.starts_with("nine") {
                replaced_value.push(9);
            } else if sliced.chars().nth(0).unwrap().is_numeric() {
                replaced_value.push(sliced.chars().nth(0).unwrap().to_digit(10).unwrap());
            }
        }
        let current_number = replaced_value[0] * 10 + replaced_value.pop().unwrap();

        final_sum += current_number;
    }
    final_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 281);
    }
}
