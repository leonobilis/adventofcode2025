use std::fs;

fn parse_input(inp: String) -> (Vec<usize>, Vec<((usize, usize), Vec<usize>)>) {
    let s = inp.split("\n\n");

    (
        s.clone()
            .take(6)
            .map(|pattern| pattern.to_string().chars().filter(|c| *c == '#').count())
            .collect(),
        s.last()
            .unwrap()
            .lines()
            .map(|region| {
                let mut s = region.split(": ");
                let mut s1 = s.next().unwrap().split("x");
                let s2 = s.next().unwrap().split(" ");

                (
                    (
                        s1.next().unwrap().parse().unwrap(),
                        s1.next().unwrap().parse().unwrap(),
                    ),
                    s2.map(|x| x.parse().unwrap()).collect(),
                )
            })
            .collect(),
    )
}
fn p1(shapes: Vec<usize>, regions: Vec<((usize, usize), Vec<usize>)>) -> usize {
    regions
        .iter()
        .filter(|((a, b), presents)| {
            a * b
                >= presents
                    .iter()
                    .enumerate()
                    .map(|(i, p)| shapes[i] * p)
                    .sum()
        })
        .count()
}

fn main() {
    let (shapes, regions) = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(shapes, regions));
}
