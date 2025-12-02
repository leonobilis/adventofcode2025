use std::fs;

fn parse_input(inp: String) -> Vec<(i64, i64)> {
    inp.split(",")
        .map(|range| {
            let mut s = range.split("-");
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn solve(inp: Vec<(i64, i64)>, check: fn(String) -> bool) -> i64 {
    inp.iter().fold(0, |sum, (a, b)| {
        sum + (*a..=*b)
            .filter(|i| check(i.to_string()))
            .fold(0, |acc, i| acc + i)
    })
}

fn p1(inp: Vec<(i64, i64)>) -> i64 {
    solve(inp, |id| {
        id.len() % 2 == 0 && id[..id.len() / 2] == id[id.len() / 2..]
    })
}

fn p2(inp: Vec<(i64, i64)>) -> i64 {
    solve(inp, |id| {
        'main: for i in 2..=id.len() {
            if id.len() % i == 0 {
                let pattern = &id[..id.len() / i];
                for j in 1..i {
                    if id[j * id.len() / i..(j + 1) * id.len() / i] != *pattern {
                        continue 'main;
                    }
                }
                return true;
            }
        }
        false
    })
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(inp.clone()));
    println!("Part 2: {}", p2(inp));
}
