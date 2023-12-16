use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
enum Object {
    Empty,
    VerticaSplitter,
    HorizontalSplitter,
    UpwardDeflector,
    DownwardDeflector,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum RayDirection {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Ray {
    position: (isize, isize),
    direction: RayDirection,
}

fn is_valid_ray(ray: &Ray, row_length: isize, col_length: isize) -> bool {
    ray.position.0 >= 0
        && ray.position.0 < row_length
        && ray.position.1 >= 0
        && ray.position.1 < col_length
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, res) = separated_list1(
        line_ending,
        many1(alt((
            combinator::map(char('.'), |_| Object::Empty),
            combinator::map(char('|'), |_| Object::VerticaSplitter),
            combinator::map(char('-'), |_| Object::HorizontalSplitter),
            combinator::map(char('/'), |_| Object::UpwardDeflector),
            combinator::map(char('\\'), |_| Object::DownwardDeflector),
        ))),
    )(input)?;
    Ok((input, res))
}

fn hash_key(ray: &Ray) -> String {
    format!(
        "{}_{}_{}",
        ray.position.0,
        ray.position.1,
        match ray.direction {
            RayDirection::Right => 0,
            RayDirection::Left => 1,
            RayDirection::Up => 2,
            RayDirection::Down => 3,
        },
    )
}

fn procress_a_ray(inital_ray: Ray, grid: &Vec<Vec<Object>>) -> u32 {
    let mut hash_set: HashSet<(isize, isize)> = HashSet::new();
    let mut rays_to_be_traced: Vec<Ray> = vec![inital_ray];

    let mut traversed_rays: HashSet<String> = HashSet::new();
    while rays_to_be_traced.len() != 0 {
        let mut current_ray = rays_to_be_traced.pop().expect("should have a value");
        traversed_rays.insert(hash_key(&current_ray));
        let mut currrent_ray_path: HashSet<String> = HashSet::new();
        while is_valid_ray(&current_ray, grid.len() as isize, grid[0].len() as isize) {
            hash_set.insert(current_ray.position);

            if currrent_ray_path.contains(&hash_key(&current_ray)) {
                break;
            }
            currrent_ray_path.insert(hash_key(&current_ray));
            match grid[current_ray.position.0 as usize][current_ray.position.1 as usize] {
                Object::Empty => match current_ray.direction {
                    RayDirection::Right => {
                        current_ray.position.1 += 1;
                    }
                    RayDirection::Left => {
                        current_ray.position.1 -= 1;
                    }
                    RayDirection::Up => {
                        current_ray.position.0 -= 1;
                    }
                    RayDirection::Down => {
                        current_ray.position.0 += 1;
                    }
                },
                Object::VerticaSplitter => match current_ray.direction {
                    RayDirection::Down => {
                        current_ray.position.0 += 1;
                    }
                    RayDirection::Up => {
                        current_ray.position.0 -= 1;
                    }
                    RayDirection::Right | RayDirection::Left => {
                        let new_ray = Ray {
                            position: (current_ray.position.0 - 1, current_ray.position.1),
                            direction: RayDirection::Up,
                        };
                        if !traversed_rays.contains(&hash_key(&new_ray)) {
                            rays_to_be_traced.push(new_ray);
                        }

                        current_ray = Ray {
                            position: (current_ray.position.0 + 1, current_ray.position.1),
                            direction: RayDirection::Down,
                        };
                    }
                },
                Object::HorizontalSplitter => match current_ray.direction {
                    RayDirection::Right => {
                        current_ray.position.1 += 1;
                    }
                    RayDirection::Left => {
                        current_ray.position.1 -= 1;
                    }
                    RayDirection::Up | RayDirection::Down => {
                        let new_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 - 1),
                            direction: RayDirection::Left,
                        };
                        if !traversed_rays.contains(&hash_key(&new_ray)) {
                            rays_to_be_traced.push(new_ray);
                        }

                        current_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 + 1),
                            direction: RayDirection::Right,
                        };
                    }
                },
                // "/"
                Object::UpwardDeflector => match current_ray.direction {
                    RayDirection::Right => {
                        current_ray = Ray {
                            position: (current_ray.position.0 - 1, current_ray.position.1),
                            direction: RayDirection::Up,
                        };
                    }
                    RayDirection::Left => {
                        current_ray = Ray {
                            position: (current_ray.position.0 + 1, current_ray.position.1),
                            direction: RayDirection::Down,
                        };
                    }
                    RayDirection::Down => {
                        current_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 - 1),
                            direction: RayDirection::Left,
                        };
                    }
                    RayDirection::Up => {
                        current_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 + 1),
                            direction: RayDirection::Right,
                        };
                    }
                },
                // "\"
                Object::DownwardDeflector => match current_ray.direction {
                    RayDirection::Right => {
                        current_ray = Ray {
                            position: (current_ray.position.0 + 1, current_ray.position.1),
                            direction: RayDirection::Down,
                        };
                    }
                    RayDirection::Left => {
                        current_ray = Ray {
                            position: (current_ray.position.0 - 1, current_ray.position.1),
                            direction: RayDirection::Up,
                        };
                    }
                    RayDirection::Down => {
                        current_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 + 1),
                            direction: RayDirection::Right,
                        };
                    }
                    RayDirection::Up => {
                        current_ray = Ray {
                            position: (current_ray.position.0, current_ray.position.1 - 1),
                            direction: RayDirection::Left,
                        };
                    }
                },
            }
        }
    }
    hash_set.len() as u32
}

pub fn process(input: &str) -> u32 {
    let start = Instant::now();
    let (_, grid) = parse_input(input).expect("should have parsed");
    // dbg!(input, grid);
    let corner_rays = (0..grid.len())
        .map(|idx| {
            vec![
                Ray {
                    position: (0, idx as isize),
                    direction: RayDirection::Down,
                },
                Ray {
                    position: (idx as isize, 0),
                    direction: RayDirection::Right,
                },
                Ray {
                    position: (grid.len() as isize - 1, idx as isize),
                    direction: RayDirection::Up,
                },
                Ray {
                    position: (idx as isize, grid.len() as isize - 1),
                    direction: RayDirection::Left,
                },
            ]
        })
        .flatten()
        .collect::<Vec<Ray>>();

    let corner_rays = corner_rays
        .into_par_iter()
        .map(|initial_ray| procress_a_ray(initial_ray, &grid))
        .max()
        .unwrap();

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    corner_rays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1_test.txt");
        let result = process(input);
        assert_eq!(result, 46);
    }
    #[test]
    fn it_works2() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 8112);
    }
}
