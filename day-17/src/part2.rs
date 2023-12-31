use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    cost: u32,
    position: (usize, usize),
    direction: PointingDirection,
    depth: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum PointingDirection {
    Right,
    Left,
    Up,
    Down,
}

impl PointingDirection {
    fn opposite(&self) -> PointingDirection {
        match self {
            PointingDirection::Right => PointingDirection::Left,
            PointingDirection::Left => PointingDirection::Right,
            PointingDirection::Up => PointingDirection::Down,
            PointingDirection::Down => PointingDirection::Up,
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.depth.cmp(&other.depth))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct DistKey {
    position: (usize, usize),
    direction: PointingDirection,
    depth: u8,
}

fn find_valid_neighbours(
    curr: State,
    path: &Vec<Vec<u32>>,
    row_limit: isize,
    col_limit: isize,
) -> Vec<State> {
    [
        (0, 1, PointingDirection::Right),
        (0, -1, PointingDirection::Left),
        (1, 0, PointingDirection::Down),
        (-1, 0, PointingDirection::Up),
    ]
    .iter()
    .filter_map(|ele| {
        let final_position = (
            curr.position.0 as isize + ele.0,
            curr.position.1 as isize + ele.1,
        );

        if !(final_position.0 >= 0
            && final_position.0 < row_limit
            && final_position.1 >= 0
            && final_position.1 < col_limit)
            || curr.direction == ele.2.opposite()
        {
            // println!("invalid node for {:?} {:?}", curr, ele);
            return None;
        }

        if (curr.depth < 4 && curr.direction != ele.2)
            || (curr.depth == 10 && curr.direction == ele.2)
        {
            // println!("invalid constraint for {:?} {:?}", curr, ele);
            return None;
        }

        Some(State {
            cost: curr.cost + path[final_position.0 as usize][final_position.1 as usize],
            position: (final_position.0 as usize, final_position.1 as usize),
            direction: ele.2,
            depth: if ele.2 == curr.direction {
                curr.depth + 1
            } else {
                1
            },
        })
    })
    // .inspect(|ele| println!("curr {:?} neigh {:?}", curr, ele))
    .collect()
}

pub fn process(input: &str) -> u32 {
    let path = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut dist: HashMap<DistKey, u32> = HashMap::new();

    let final_position = (path.len() - 1, path[0].len() - 1);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    dist.insert(
        DistKey {
            position: (0, 0),
            direction: PointingDirection::Right,
            depth: 1,
        },
        0,
    );
    dist.insert(
        DistKey {
            position: (0, 0),
            direction: PointingDirection::Down,
            depth: 1,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        position: (0, 0),
        direction: PointingDirection::Right,
        depth: 1,
    });

    let mut ans = u32::MAX;

    while let Some(curr_state) = heap.pop() {
        let dist_key = DistKey {
            position: curr_state.position,
            direction: curr_state.direction,
            depth: curr_state.depth,
        };

        //We might have found better way
        if dist.contains_key(&dist_key) && curr_state.cost > dist[&dist_key] {
            println!("found better way");
            continue;
        }

        // println!("current ele {:?}", curr_state);
        find_valid_neighbours(
            curr_state,
            &path,
            path.len() as isize,
            path[0].len() as isize,
        )
        .iter()
        // .inspect(|ele| {
        //     println!("neighbours {:?}", ele);
        // })
        .for_each(|ele| {
            let dist_key = DistKey {
                position: ele.position,
                direction: ele.direction,
                depth: ele.depth,
            };
            if ele.position == final_position && ele.depth >= 4 {
                ans = std::cmp::min(ans, ele.cost);
            }
            if !dist.contains_key(&dist_key)
                || ele.cost < dist[&dist_key]
                || (ele.position == final_position && ele.depth >= 4)
            {
                heap.push(ele.clone());

                dist.insert(dist_key, ele.cost);
            }
        })
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
        assert_eq!(result, 1320);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 498538);
    }
}
