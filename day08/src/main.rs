use std::collections::{HashMap, HashSet};
use std::{fs, usize};

fn parse_input(inp: String) -> Vec<(isize, isize, isize)> {
    inp.lines()
        .map(|line| {
            let mut s = line.split(",");
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn get_dists(
    inp: &Vec<(isize, isize, isize)>,
) -> Vec<((isize, isize, isize), (isize, isize, isize), f64)> {
    let mut dists: Vec<_> = inp
        .iter()
        .enumerate()
        .map(|(i, (x, y, z))| {
            inp.iter().skip(i + 1).map(move |(x2, y2, z2)| {
                (
                    (*x, *y, *z),
                    (*x2, *y2, *z2),
                    (((x - x2).pow(2) + (y - y2).pow(2) + (z - z2).pow(2)) as f64).sqrt(),
                )
            })
        })
        .flatten()
        .collect();
    dists.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
    dists
}

fn solve(
    dists: impl Iterator<Item = ((isize, isize, isize), (isize, isize, isize), f64)>,
) -> (
    HashMap<usize, HashSet<(isize, isize, isize)>>,
    ((isize, isize, isize), (isize, isize, isize)),
) {
    let mut circuits = HashMap::<usize, HashSet<(isize, isize, isize)>>::new();

    let mut next_id = 0;
    let mut last = ((0, 0, 0), (0, 0, 0));

    for (a, b, _) in dists {
        let mut seta: Option<usize> = None;
        let mut setb: Option<usize> = None;

        for (id, circuit) in circuits.iter() {
            if circuit.contains(&a) {
                seta = Some(*id)
            }
            if circuit.contains(&b) {
                setb = Some(*id)
            }
        }

        match (seta, setb) {
            (Some(ida), Some(idb)) => {
                if ida != idb {
                    let circuitb = circuits.remove(&idb).unwrap();
                    circuits.entry(ida).and_modify(|circuita| {
                        circuita.extend(circuitb);
                    });
                    last = (a, b);
                }
            }
            (Some(ida), None) => {
                circuits.entry(ida).and_modify(|circuit| {
                    circuit.insert(b);
                });
                last = (a, b);
            }
            (None, Some(idb)) => {
                circuits.entry(idb).and_modify(|circuit| {
                    circuit.insert(a);
                });
                last = (a, b);
            }
            (None, None) => {
                circuits.insert(next_id, HashSet::from([a, b]));
                next_id += 1;
            }
        }
    }

    (circuits, last)
}

fn p1(dists: &Vec<((isize, isize, isize), (isize, isize, isize), f64)>) -> usize {
    let (circuits, _) = solve(dists.iter().take(1000).cloned());
    let mut circuits_vec: Vec<_> = circuits.iter().map(|(_, v)| v.len()).collect();
    circuits_vec.sort();
    circuits_vec.iter().rev().take(3).fold(1, |sum, x| sum * x)
}

fn p2(dists: Vec<((isize, isize, isize), (isize, isize, isize), f64)>) -> isize {
    let (_, last) = solve(dists.into_iter());
    last.0.0 * last.1.0
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());
    let dists = get_dists(&inp);

    println!("Part 1: {}", p1(&dists));
    println!("Part 2: {}", p2(dists));
}
