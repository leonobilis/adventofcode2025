use std::fs;

fn parse_input(inp: String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut splitted = inp.split("\n\n");

    (
        splitted
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut s = line.split("-");
                (
                    s.next().unwrap().parse().unwrap(),
                    s.next().unwrap().parse().unwrap(),
                )
            })
            .collect(),
        splitted
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}

fn p1(ranges: &Vec<(u64, u64)>, ids: Vec<u64>) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|(a, b)| **id >= *a && **id <= *b))
        .count()
}

fn p2(ranges: Vec<(u64, u64)>) -> u64 {
    let mut actual = Vec::new();

    for (a, b) in ranges {
        let mut new = Vec::new();
        let mut a1 = a;
        let mut b1 = b;

        for (a2, b2) in actual {
            if a >= a2 && b <= b2 {
                a1 = a2;
                b1 = b2;
            } else if a < a2 && b >= a2 && b <= b2 {
                b1 = b2;
            } else if a >= a2 && a <= b2 && b > b2 {
                a1 = a2;
            } else if a > b2 || b < a2 {
                new.push((a2, b2));
            }
        }
        new.push((a1, b1));

        actual = new;
    }

    actual.iter().fold(0, |sum, (a, b)| sum + b - a + 1)
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(&inp.0, inp.1));
    println!("Part 2: {}", p2(inp.0));
}
