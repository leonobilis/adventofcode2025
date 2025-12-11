use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, hash_map::RandomState};
use std::fs;

fn parse_input(inp: String) -> HashMap<String, Vec<String>> {
    inp.lines()
        .map(|line| {
            let mut s = line.split(" ");
            (
                s.next().unwrap().strip_suffix(":").unwrap().to_string(),
                s.into_iter().map(str::to_string).collect(),
            )
        })
        .collect()
}

fn p1(inp: &HashMap<String, Vec<String>>) -> usize {
    let mut g = DiGraph::<String, ()>::new();

    let mut nodes: HashMap<String, NodeIndex> = inp
        .iter()
        .map(|(device, _)| (device.clone(), g.add_node(device.clone())))
        .collect();
    nodes.insert("out".to_string(), g.add_node("out".to_string()));

    for (device, outputs) in inp {
        for o in outputs {
            g.add_edge(*nodes.get(device).unwrap(), *nodes.get(o).unwrap(), ());
        }
    }

    all_simple_paths::<Vec<_>, _, RandomState>(
        &g,
        *nodes.get("you").unwrap(),
        *nodes.get("out").unwrap(),
        0,
        None,
    )
    .count()
}

fn traverse(
    device: String,
    fft: bool,
    dac: bool,
    inp: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    if device == "out" {
        match (fft, dac) {
            (true, true) => 1,
            _ => 0,
        }
    } else {
        match cache.get(&(device.clone(), fft, dac)) {
            Some(x) => *x,
            None => {
                let val = inp
                    .get(&device)
                    .unwrap()
                    .iter()
                    .map(|d| traverse(d.clone(), fft || d == "fft", dac || d == "dac", inp, cache))
                    .sum();
                cache.insert((device, fft, dac), val);
                val
            }
        }
    }
}

fn p2(inp: &HashMap<String, Vec<String>>) -> u64 {
    traverse(
        "svr".to_string(),
        false,
        false,
        inp,
        &mut HashMap::<(String, bool, bool), u64>::new(),
    )
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(&inp));
    println!("Part 2: {}", p2(&inp));
}
