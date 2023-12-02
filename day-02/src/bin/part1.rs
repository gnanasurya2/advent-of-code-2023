fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let game_limit = (12, 13, 14);
    let val: Vec<i32> = input
        .split("\r\n")
        .map(|line| {
            let mut iterator = line.split(":");
            let game_id = iterator
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let ball_counts: Vec<(i32, i32, i32)> = iterator
                .next()
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

            let res: Result<i32, &str> = ball_counts.iter().try_fold(1i32, |acc, &x| {
                if acc != 0 {
                    if game_limit.0 >= x.0 && game_limit.1 >= x.1 && game_limit.2 >= x.2 {
                        Ok(game_id)
                    } else {
                        Ok(0)
                    }
                } else {
                    Ok(acc)
                }
            });

            match res {
                Ok(val) => val,
                Err(_) => 0,
            }
        })
        .collect();
    val.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = part1(input);
        assert_eq!(result, 8);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, 3059);
    }
}
