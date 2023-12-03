pub fn process(input: &str) -> u32 {
    let mut final_sum = 0;
    for calibration_value in input.split("\r\n") {
        let len = calibration_value.len();
        let chars: Vec<char> = calibration_value.chars().collect();
        let mut found_first_number = false;
        let mut found_last_number = false;
        let mut current_number = 0;
        for i in 0..len {
            if !found_first_number && chars[i].is_numeric() {
                found_first_number = true;
                current_number += chars[i].to_digit(10).unwrap() * 10;
            }

            if !found_last_number && chars[len - 1 - i].is_numeric() {
                found_last_number = true;
                current_number += chars[len - 1 - i].to_digit(10).unwrap();
            }
        }
        // println!("{} {}", calibration_value, current_number);
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
        assert_eq!(result, 142);
    }
}
