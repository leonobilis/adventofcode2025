use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(inp: String) -> ((usize, usize), HashSet<(usize, usize)>) {
    let mut lines = inp.lines();

    (
        (lines.next().unwrap().find('S').unwrap(), 0),
        inp.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '^' => Some((x, y)),
                    _ => None,
                })
            })
            .flatten()
            .collect(),
    )
}

fn p1(start: (usize, usize), splitters: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;
    let end = splitters.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut actual = HashSet::new();
    actual.insert(start);

    for _ in 0..end {
        let mut splits = Vec::<(usize, usize)>::new();

        let mut new: HashSet<(usize, usize)> = actual
            .iter()
            .filter_map(|(x, y)| {
                if splitters.contains(&(*x, *y + 1)) {
                    splits.push((*x, *y + 1));
                    None
                } else {
                    Some((*x, *y + 1))
                }
            })
            .collect();

        count += splits.len();

        for (x, y) in splits {
            new.insert((x - 1, y));
            new.insert((x + 1, y));
        }

        actual = new
    }

    count
}

fn p2(start: (usize, usize), splitters: &HashSet<(usize, usize)>) -> usize {
    let end = splitters.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut actual = HashMap::new();
    actual.insert(start, 1);

    for _ in 0..end {
        let mut splits = Vec::<((usize, usize), usize)>::new();

        let mut new: HashMap<(usize, usize), usize> = actual
            .iter()
            .filter_map(|((x, y), count)| {
                if splitters.contains(&(*x, *y + 1)) {
                    splits.push(((*x, *y + 1), *count));
                    None
                } else {
                    Some(((*x, *y + 1), *count))
                }
            })
            .collect();

        for ((x, y), v) in splits {
            new.entry((x - 1, y))
                .and_modify(|count| *count += v)
                .or_insert(v);
            new.entry((x + 1, y))
                .and_modify(|count| *count += v)
                .or_insert(v);
        }

        actual = new
    }

    actual.into_values().fold(0, |sum, v| sum + v)
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(inp.0, &inp.1));
    println!("Part 2: {}", p2(inp.0, &inp.1));
}
