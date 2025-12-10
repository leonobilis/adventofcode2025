use good_lp::{Expression, ProblemVariables, Solution, SolverModel, microlp, variable};
use std::{collections::HashSet, fs};

fn parse_input(inp: String) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    inp.lines()
        .map(|line| {
            let mut s = line.split(" ");
            (
                s.next()
                    .unwrap()
                    .chars()
                    .filter_map(|c| match c {
                        '.' => Some(false),
                        '#' => Some(true),
                        _ => None,
                    })
                    .collect(),
                s.clone()
                    .take_while(|x| x.starts_with("("))
                    .map(|b| {
                        //let l = b.len();
                        b[1..b.len() - 1]
                            .split(",")
                            .map(|b2| b2.parse().unwrap())
                            .collect()
                    })
                    .collect(),
                match s.last() {
                    Some(j) => j[1..j.len() - 1]
                        .split(",")
                        .map(|j2| j2.parse().unwrap())
                        .collect(),
                    None => panic!("wrong data"),
                },
            )
        })
        .collect()
}

fn p1(inp: &Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>) -> usize {
    inp.iter()
        .map(|(lights_pattern, buttons, _)| {
            let mut seen = HashSet::<Vec<bool>>::new();
            seen.insert(vec![false; lights_pattern.len()]);
            let mut to_check = vec![vec![false; lights_pattern.len()]];
            let mut iter = 0;

            loop {
                iter += 1;
                let mut to_check_new = Vec::<Vec<bool>>::new();
                for lights in to_check {
                    for button in buttons {
                        let mut lights_new = lights.clone();
                        for b in button {
                            if lights[*b] {
                                lights_new[*b] = false;
                            } else {
                                lights_new[*b] = true;
                            }
                        }
                        if lights_new == *lights_pattern {
                            return iter;
                        }

                        if !seen.contains(&lights_new) {
                            seen.insert(lights_new.clone());
                            to_check_new.push(lights_new);
                        }
                    }
                }
                to_check = to_check_new;
            }
        })
        .fold(0, |sum, x| sum + x)
}

fn p2(inp: &Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>) -> f64 {
    inp.iter()
        .map(|(_, buttons, joltage_pattern)| {
            let mut problem = ProblemVariables::new();
            let vars = problem.add_vector(variable().integer().min(0), buttons.len());

            let objective: Expression = vars.iter().sum();
            let mut model = problem.minimise(objective).using(microlp);

            for (i, j) in joltage_pattern.iter().enumerate() {
                let v: Vec<_> = buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(bi, button)| {
                        if button.contains(&i) {
                            Some(vars[bi])
                        } else {
                            None
                        }
                    })
                    .collect();
                let e: Expression = v.iter().sum();
                model.add_constraint(e.eq(*j as f64));
            }

            let s = model.solve().unwrap();

            vars.iter().map(|&var| s.value(var)).sum()
        })
        .fold(0., |sum, x: f64| sum + x)
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(&inp));
    println!("Part 2: {}", p2(&inp));
}
