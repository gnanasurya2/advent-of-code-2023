#![allow(dead_code)]

use regex::Regex;

const RE: &str = r#"(\w) (\d+) \(#(\S+)(\d)\)"#;

pub fn process(input: &str) -> i64 {
    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary = 1;
    for [dir, steps, ..] in parse(input, &re) {
        let steps = steps.parse().unwrap();
        curr = proceed(curr, dir.as_bytes()[0], steps);
        println!("{:?}", curr);
        vertices.push(curr);
        boundary += steps;
    }
    solve(&vertices, boundary)
}

// fn p2(input: &str) -> i64 {
//     let re = Regex::new(RE).unwrap();
//     let mut curr = (0, 0);
//     let mut vertices = vec![curr];
//     let mut boundary = 1;
//     for [.., steps, dir] in parse(input, &re) {
//         let steps = i64::from_str_radix(steps, 16).unwrap();
//         curr = proceed(curr, dir.as_bytes()[0], steps);
//         vertices.push(curr);
//         boundary += steps;
//     }
//     solve(&vertices, boundary)
// }

fn solve(vertices: &[(i64, i64)], boundary: i64) -> i64 {
    // Shoelace formula
    // https://www.themathdoctors.org/polygon-coordinates-and-areas/
    let area: i64 = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    area.abs() / 2 + boundary / 2 + 1
}

const fn proceed(curr: (i64, i64), dir: u8, steps: i64) -> (i64, i64) {
    match dir {
        b'U' | b'3' => (curr.0, curr.1 - steps),
        b'D' | b'1' => (curr.0, curr.1 + steps),
        b'R' | b'0' => (curr.0 + steps, curr.1),
        b'L' | b'2' => (curr.0 - steps, curr.1),
        _ => unreachable!(),
    }
}

fn parse<'a>(input: &'a str, re: &'a Regex) -> impl Iterator<Item = [&'a str; 4]> {
    re.captures_iter(input).map(|cap| {
        let (_, group) = cap.extract::<4>();
        group
    })
}
