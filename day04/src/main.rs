use std::collections::HashSet;
use std::fs;

fn parse_input(inp: String) -> HashSet<(isize, isize)> {
    inp.lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, val)| **val == b'@')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .flatten()
        .collect()
}

fn is_accessible(x: isize, y: isize, inp: &HashSet<(isize, isize)>) -> bool {
    let mut count = 0;

    if inp.contains(&(x + 1, y)) {
        count += 1;
    }
    if inp.contains(&(x - 1, y)) {
        count += 1;
    }
    if inp.contains(&(x, y + 1)) {
        count += 1;
    }
    if inp.contains(&(x + 1, y + 1)) {
        count += 1;
    }
    if inp.contains(&(x - 1, y + 1)) {
        count += 1;
    }
    if inp.contains(&(x, y - 1)) {
        count += 1;
    }
    if inp.contains(&(x + 1, y - 1)) {
        count += 1;
    }
    if inp.contains(&(x - 1, y - 1)) {
        count += 1;
    }

    count < 4
}

fn p1(inp: HashSet<(isize, isize)>) -> usize {
    let cloned_inp = inp.clone();
    inp.iter()
        .filter(|(x, y)| is_accessible(*x, *y, &cloned_inp))
        .count()
}

fn p2(mut inp: HashSet<(isize, isize)>) -> usize {
    let mut count = 0;

    loop {
        let cloned_inp = inp.clone();
        let to_remove: Vec<&(isize, isize)> = cloned_inp
            .iter()
            .filter(|(x, y)| is_accessible(*x, *y, &cloned_inp))
            .collect();

        if to_remove.is_empty() {
            break;
        }

        count += to_remove.len();

        to_remove.iter().for_each(|pos| {
            inp.remove(pos);
        });
    }

    count
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(inp.clone()));
    println!("Part 2: {}", p2(inp));
}
