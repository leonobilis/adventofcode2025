use std::collections::HashMap;
use std::fs;

fn parse_input(inp: String) -> Vec<(i64, i64)> {
    inp.lines()
        .map(|line| {
            let mut s = line.split(",");
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn p1(inp: &Vec<(i64, i64)>) -> i64 {
    inp.iter()
        .enumerate()
        .map(|(i, (x, y))| {
            inp.iter()
                .skip(i + 1)
                .map(move |(x2, y2)| ((x - x2).abs() + 1) * ((y - y2).abs() + 1))
        })
        .flatten()
        .max()
        .unwrap()
}

fn p2(inp: &Vec<(i64, i64)>) -> i64 {
    let mut xs = HashMap::<i64, (i64, i64)>::new();
    let mut ys = HashMap::<i64, (i64, i64)>::new();

    inp.iter().for_each(|(x, y)| {
        xs.entry(*x)
            .and_modify(|v| {
                if *y < v.0 {
                    *v = (*y, v.1);
                } else if *y > v.1 {
                    *v = (v.0, *y)
                }
            })
            .or_insert((*y, *y));
        ys.entry(*y)
            .and_modify(|v| {
                if *x < v.0 {
                    *v = (*x, v.1);
                } else if *x > v.1 {
                    *v = (v.0, *x)
                }
            })
            .or_insert((*x, *x));
    });

    let ykeys: Vec<_> = ys.keys().cloned().collect();

    xs.iter().for_each(|(x, (min, max))| {
        ykeys
            .iter()
            .filter(|k| *k >= min && *k <= max)
            .for_each(|k| {
                ys.entry(*k).and_modify(|v| {
                    if *x < v.0 {
                        *v = (*x, v.1);
                    } else if *x > v.1 {
                        *v = (v.0, *x)
                    }
                });
            });
    });

    inp.iter()
        .enumerate()
        .map(|(i, (x, y))| {
            inp.iter()
                .skip(i + 1)
                .map(move |(x2, y2)| ((x, y), (x2, y2)))
                .filter_map(|((x, y), (x2, y2))| {
                    let (mut xh, mut xl) = (x, x2);
                    if x2 > x {
                        (xh, xl) = (x2, x)
                    }
                    let (mut yh, mut yl) = (y, y2);
                    if y2 > y {
                        (yh, yl) = (y2, y)
                    }

                    if ykeys.iter().filter(|k| *k >= yl && *k <= yh).all(|y| {
                        let (min, max) = ys.get(&y).unwrap();
                        min <= xl && max >= xh
                    }) {
                        Some((xh - xl + 1) * (yh - yl + 1))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .max()
        .unwrap()
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(&inp));
    println!("Part 2: {}", p2(&inp));
}
