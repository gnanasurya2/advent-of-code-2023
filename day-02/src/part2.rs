use std::cmp;

pub fn process(input: &str) -> i32 {
    let val: Vec<i32> = input
        .split("\r\n")
        .map(|line| {
            let ball_counts: Vec<(i32, i32, i32)> = line
                .split(":")
                .last()
                .unwrap()
                .split(";")
                .map(|game| {
                    let mut current_balls_count = (0, 0, 0);
                    for count in game.split(",") {
                        let mut count_iterator = count.trim().split(" ");
                        let ball_count = count_iterator
                            .next()
                            .unwrap()
                            .trim()
                            .parse::<i32>()
                            .unwrap();
                        let color = count_iterator.next().unwrap().trim();
                        if color == "red" {
                            current_balls_count.0 = ball_count;
                        } else if color == "green" {
                            current_balls_count.1 = ball_count;
                        } else {
                            current_balls_count.2 = ball_count;
                        }
                    }
                    current_balls_count
                })
                .collect();

            let res: Result<(i32, i32, i32), &str> =
                ball_counts.iter().try_fold((0, 0, 0), |acc, &x| {
                    Ok((
                        cmp::max(acc.0, x.0),
                        cmp::max(acc.1, x.1),
                        cmp::max(acc.2, x.2),
                    ))
                });

            match res {
                Ok(val) => val.0 * val.1 * val.2,
                Err(_) => 0,
            }
        })
        .collect();
    val.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 2286);
    }

    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 65371);
    }
}
