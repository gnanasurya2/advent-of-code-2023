#[derive(Debug, PartialEq, Eq)]
enum MapElements {
    Empty,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthEastPipe,
    SouthWestPipe,
    StartingPipe,
    Visited,
}

fn find_valid_neighbours(
    input: &mut Vec<Vec<MapElements>>,
    current_location: (usize, usize),
    row_limit: usize,
    column_limit: usize,
) -> Vec<(usize, usize)> {
    let south_valid_connections = [
        MapElements::VerticalPipe,
        MapElements::SouthEastPipe,
        MapElements::SouthWestPipe,
    ];
    let north_valid_connections = [
        MapElements::VerticalPipe,
        MapElements::NorthEastPipe,
        MapElements::NorthWestPipe,
    ];

    let east_valid_connections = [
        MapElements::HorizontalPipe,
        MapElements::SouthWestPipe,
        MapElements::NorthWestPipe,
    ];

    let west_valid_connections = [
        MapElements::HorizontalPipe,
        MapElements::SouthEastPipe,
        MapElements::NorthEastPipe,
    ];

    let mut valid_neighbours = vec![];

    match input[current_location.0][current_location.1] {
        MapElements::VerticalPipe => {
            if current_location.0 as isize - 1 >= 0
                && south_valid_connections
                    .contains(&input[current_location.0 - 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 - 1, current_location.1))
            } else if current_location.0 + 1 < row_limit
                && north_valid_connections
                    .contains(&input[current_location.0 + 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 + 1, current_location.1))
            }
        }
        MapElements::HorizontalPipe => {
            if current_location.1 + 1 < column_limit
                && east_valid_connections
                    .contains(&input[current_location.0][current_location.1 + 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 + 1))
            } else if current_location.1 as isize - 1 >= 0
                && west_valid_connections
                    .contains(&input[current_location.0][current_location.1 - 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 - 1))
            }
        }
        MapElements::NorthEastPipe => {
            if current_location.0 as isize - 1 >= 0
                && south_valid_connections
                    .contains(&input[current_location.0 - 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 - 1, current_location.1))
            } else if current_location.1 + 1 < column_limit
                && east_valid_connections
                    .contains(&input[current_location.0][current_location.1 + 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 + 1))
            }
        }
        MapElements::NorthWestPipe => {
            if current_location.0 as isize - 1 >= 0
                && south_valid_connections
                    .contains(&input[current_location.0 - 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 - 1, current_location.1))
            } else if current_location.1 as isize - 1 >= 0
                && west_valid_connections
                    .contains(&input[current_location.0][current_location.1 - 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 - 1))
            }
        }
        MapElements::SouthWestPipe => {
            if current_location.1 as isize - 1 >= 0
                && west_valid_connections
                    .contains(&input[current_location.0][current_location.1 - 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 - 1))
            } else if current_location.0 + 1 < column_limit
                && north_valid_connections
                    .contains(&input[current_location.0 + 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 + 1, current_location.1))
            }
        }
        MapElements::SouthEastPipe => {
            if current_location.1 + 1 < row_limit
                && east_valid_connections
                    .contains(&input[current_location.0][current_location.1 + 1])
            {
                valid_neighbours.push((current_location.0, current_location.1 + 1))
            } else if current_location.0 + 1 < column_limit
                && north_valid_connections
                    .contains(&input[current_location.0 + 1][current_location.1])
            {
                valid_neighbours.push((current_location.0 + 1, current_location.1))
            }
        }
        _ => println!("shouldn't happen {:?}", current_location),
    }

    valid_neighbours
}

fn traverse_map(
    input: &mut Vec<Vec<MapElements>>,
    current_location: (usize, usize),
    _depth: u32,
    row_limit: usize,
    column_limit: usize,
) -> u32 {
    let mut dep = 0;
    let mut current_loc = current_location;
    loop {
        let neighbour = find_valid_neighbours(input, current_loc, row_limit, column_limit);
        if neighbour.len() == 0 {
            return dep;
        }
        input[current_loc.0][current_loc.1] = MapElements::Visited;
        dep += 1;
        current_loc = (neighbour[0].0, neighbour[0].1)
    }
}

fn find_enclosed_region(
    input: &mut Vec<Vec<MapElements>>,
    row_limit: isize,
    column_limit: isize,
) -> u32 {
    let mut ans = 0;
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];

    while stack.len() != 0 {
        let curr = stack.pop().unwrap();

        if input[curr.0][curr.1] == MapElements::Visited {
            continue;
        }
        let mut valid_neighbours: Vec<(usize, usize)> = [
            (curr.0 as isize, curr.1 as isize + 1),
            (curr.0 as isize, curr.1 as isize - 1),
            (curr.0 as isize + 1, curr.1 as isize),
            (curr.0 as isize - 1, curr.1 as isize),
        ]
        .iter()
        .filter_map(|ele| {
            match ele.0 >= 0 && ele.0 < row_limit && ele.1 >= 0 && ele.1 < column_limit {
                true => Some((ele.0 as usize, ele.1 as usize)),
                false => None,
            }
        })
        .collect();

        if valid_neighbours.len() == 4 && input[curr.0][curr.1] == MapElements::Empty {
            // println!("{}  {:?}", ans, curr);
            ans += 1;
        }

        input[curr.0][curr.1] = MapElements::Visited;
        stack.append(&mut valid_neighbours);
    }
    32
}
pub fn process(input: &str) -> u32 {
    let mut starting_point = (0, 0);
    let mut game_input = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ele)| match ele {
                    '|' => MapElements::VerticalPipe,
                    '-' => MapElements::HorizontalPipe,
                    'L' => MapElements::NorthEastPipe,
                    'J' => MapElements::NorthWestPipe,
                    '7' => MapElements::SouthWestPipe,
                    'F' => MapElements::SouthEastPipe,
                    'S' => {
                        starting_point = (i, j);
                        MapElements::StartingPipe
                    }
                    _ => MapElements::Empty,
                })
                .collect::<Vec<MapElements>>()
        })
        .collect::<Vec<Vec<MapElements>>>();

    let row_limit = game_input.len();
    let column_limit = game_input[0].len();

    game_input[starting_point.0][starting_point.1] = MapElements::VerticalPipe;

    (traverse_map(&mut game_input, starting_point, 0, row_limit, column_limit) as f64 / 2.0).ceil()
        as u32;
    let mut game_input = game_input
        .into_iter()
        .map(|line| {
            line.iter()
                .map(|ele| match ele {
                    MapElements::Visited => MapElements::Visited,
                    _ => MapElements::Empty,
                })
                .collect::<Vec<MapElements>>()
        })
        .collect::<Vec<Vec<MapElements>>>();
    game_input.iter().for_each(|line| {
        line.iter().for_each(|ele| {
            print!(
                "{}",
                if ele == &MapElements::Visited {
                    "V"
                } else {
                    "."
                }
            )
        });
        println!("");
    });

    find_enclosed_region(&mut game_input, row_limit as isize, column_limit as isize);
    let ans = game_input
        .iter()
        .flat_map(|x| {
            x.iter()
                .filter(|ele| **ele == MapElements::Empty)
                .collect::<Vec<&MapElements>>()
        })
        .count() as u32;

    game_input.iter().for_each(|line| {
        line.iter().for_each(|ele| {
            print!(
                "{}",
                if ele == &MapElements::Visited {
                    "V"
                } else {
                    // print!("{} {:?}", ans, ele);
                    "A"
                }
            )
        });
        println!("");
    });
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn find_neighbours() {
        let mut input = vec![
            vec![
                MapElements::Empty,
                MapElements::VerticalPipe,
                MapElements::Empty,
            ],
            vec![
                MapElements::NorthEastPipe,
                MapElements::VerticalPipe,
                MapElements::SouthWestPipe,
            ],
            vec![MapElements::Empty, MapElements::Visited, MapElements::Empty],
        ];
        let result = find_valid_neighbours(&mut input, (1, 1), 3, 3);
        assert_eq!(result, [(0, 1)]);
    }
    #[test]
    fn find_neighbours1() {
        let mut input = vec![
            vec![
                MapElements::Empty,
                MapElements::NorthWestPipe,
                MapElements::Empty,
            ],
            vec![
                MapElements::NorthEastPipe,
                MapElements::SouthEastPipe,
                MapElements::SouthWestPipe,
            ],
            vec![
                MapElements::Empty,
                MapElements::VerticalPipe,
                MapElements::Empty,
            ],
        ];
        let result = find_valid_neighbours(&mut input, (1, 1), 3, 3);
        assert_eq!(result, [(1, 2)]);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 6717);
    }
}
