use itertools::Itertools;
use std::collections::HashMap;
#[derive(Debug)]
struct GameInput {
    value: Vec<char>,
    bid: u64,
}

pub fn process(input: &str) -> u64 {
    let card_strength: Vec<char> = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    let game_input: Vec<GameInput> = input
        .lines()
        .map(|val| {
            let line_values: Vec<&str> = val.split(" ").collect();
            GameInput {
                value: line_values[0].chars().collect(),
                bid: line_values[1].parse::<u64>().unwrap(),
            }
        })
        .collect();

    let mut five_of_a_kind: Vec<GameInput> = vec![];
    let mut four_of_a_kind: Vec<GameInput> = vec![];
    let mut full_house: Vec<GameInput> = vec![];
    let mut three_of_a_kind: Vec<GameInput> = vec![];
    let mut two_pair: Vec<GameInput> = vec![];
    let mut one_pair: Vec<GameInput> = vec![];
    let mut high_card: Vec<GameInput> = vec![];

    for hand in game_input {
        let frequencies: HashMap<char, usize> = hand.value.clone().into_iter().counts();
        let mut freq_len = frequencies.len();
        let mut max_freq = *frequencies
            .iter()
            .filter_map(|val| if val.0 == &'J' { None } else { Some(val.1) })
            .max()
            .unwrap_or_else(|| &5);

        if frequencies.contains_key(&'J') && freq_len != 1 {
            max_freq += frequencies.get(&'J').unwrap();
            freq_len -= 1;
        }

        match max_freq {
            5 => five_of_a_kind.push(hand),
            4 => four_of_a_kind.push(hand),
            3 => {
                if freq_len == 2 {
                    full_house.push(hand)
                } else {
                    three_of_a_kind.push(hand)
                }
            }
            2 => {
                if freq_len == 3 {
                    two_pair.push(hand);
                } else {
                    one_pair.push(hand);
                }
            }
            _ => high_card.push(hand),
        }
    }

    let compare_closure = |a: &GameInput, b: &GameInput| {
        for i in 0..5 {
            if a.value[i] != b.value[i] {
                return card_strength
                    .iter()
                    .position(|e| *e == b.value[i])
                    .unwrap()
                    .cmp(&card_strength.iter().position(|e| *e == a.value[i]).unwrap());
            }
        }
        a.bid.cmp(&b.bid)
    };

    five_of_a_kind.sort_by(compare_closure);

    four_of_a_kind.sort_by(compare_closure);

    full_house.sort_by(compare_closure);

    three_of_a_kind.sort_by(compare_closure);

    two_pair.sort_by(compare_closure);

    one_pair.sort_by(compare_closure);

    high_card.sort_by(compare_closure);

    let mut ans: u64 = 0;
    let mut rank: u64 = 1;
    for game in high_card {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in one_pair {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in two_pair {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in three_of_a_kind {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in full_house {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in four_of_a_kind {
        ans += rank * game.bid;
        rank += 1;
    }

    for game in five_of_a_kind {
        ans += rank * game.bid;
        rank += 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 5905);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 249631254);
    }
}
