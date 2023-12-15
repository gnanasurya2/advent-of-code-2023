use std::vec;

fn handle_dash(container: &mut Vec<(Vec<char>, u32)>, label: &Vec<char>) {
    for index in 0..container.len() {
        if container[index].0 == *label {
            container.remove(index);
            return;
        }
    }
}

fn handle_equal(container: &mut Vec<(Vec<char>, u32)>, label: &Vec<char>, focal_length: u32) {
    for index in 0..container.len() {
        if container[index].0 == *label {
            container[index].1 = focal_length;
            return;
        }
    }

    container.insert(0, (label.clone(), focal_length));
}

pub fn process(input: &str) -> usize {
    let mut lens: Vec<Vec<(Vec<char>, u32)>> = vec![vec![]; 256];
    input.split(",").for_each(|hash_input| {
        let mut sum: u32 = 0;
        let mut curr: Vec<char> = vec![];
        for ch in hash_input.chars() {
            if ch.is_alphabetic() {
                sum += ch as u32;
                curr.push(ch);
                sum *= 17;
                sum = sum % 256
            } else if ch == '-' {
                handle_dash(&mut lens[sum as usize], &curr)
            } else if ch.is_numeric() {
                handle_equal(
                    &mut lens[sum as usize],
                    &curr,
                    ch.to_digit(10).expect("should be a number"),
                )
            }
        }
    });
    lens.iter()
        .enumerate()
        .map(|(index, container)| {
            container.iter().rev().enumerate().fold(0usize, |acc, ele| {
                acc + (index + 1) * (ele.0 + 1) * ele.1 .1 as usize
            })
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
        assert_eq!(result, 145);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 286278);
    }
}
