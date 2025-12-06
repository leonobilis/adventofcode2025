use std::fs;

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Option::Some(Operation::Add),
            '*' => Option::Some(Operation::Multiply),
            _ => None,
        }
    }
}

fn parse_input1(inp: &String) -> Vec<(Operation, u64, u64, u64, u64)> {
    let mut lines = inp.lines();

    let mut a = lines.next().unwrap().split_whitespace();
    let mut b = lines.next().unwrap().split_whitespace();
    let mut c = lines.next().unwrap().split_whitespace();
    let mut d = lines.next().unwrap().split_whitespace();
    let op = lines.next().unwrap().split_whitespace();

    op.map(|oper| {
        (
            Operation::from_char(oper.chars().nth(0).unwrap()).unwrap(),
            a.next().unwrap().parse().unwrap(),
            b.next().unwrap().parse().unwrap(),
            c.next().unwrap().parse().unwrap(),
            d.next().unwrap().parse().unwrap(),
        )
    })
    .collect()
}

fn parse_input2(inp: &String) -> Vec<(Operation, u64, u64, u64, u64)> {
    let mut lines = inp.lines();

    let a: Vec<char> = lines.next().unwrap().chars().collect();
    let b: Vec<char> = lines.next().unwrap().chars().collect();
    let c: Vec<char> = lines.next().unwrap().chars().collect();
    let d: Vec<char> = lines.next().unwrap().chars().collect();
    let ops = lines.next().unwrap().to_string() + " ";
    let op = ops.chars().enumerate();

    let get_args = |o: Operation, pos: usize, len: usize| -> (Operation, u64, u64, u64, u64) {
        let mut args: Vec<u64> = (pos..pos + len)
            .map(|p| {
                let mut s = String::new();
                if a[p] != ' ' {
                    s.push(a[p]);
                }
                if b[p] != ' ' {
                    s.push(b[p]);
                }
                if c[p] != ' ' {
                    s.push(c[p]);
                }
                if d[p] != ' ' {
                    s.push(d[p]);
                }
                s.parse().unwrap()
            })
            .collect();

        for _ in 0..4 - len {
            args.push(match o {
                Operation::Add => 0,
                Operation::Multiply => 1,
            });
        }

        (o, args[0], args[1], args[2], args[3])
    };

    op.clone()
        .filter_map(|(i, o)| match Operation::from_char(o) {
            Some(o) => Option::Some(get_args(
                o,
                i,
                op.clone()
                    .skip(i + 1)
                    .take_while(|(_, x)| *x == ' ')
                    .count(),
            )),
            None => None,
        })
        .collect()
}

fn solve(inp: Vec<(Operation, u64, u64, u64, u64)>) -> u64 {
    inp.iter().fold(0, |sum, (op, a, b, c, d)| match op {
        Operation::Add => sum + a + b + c + d,
        Operation::Multiply => sum + a * b * c * d,
    })
}

fn main() {
    let inp = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve(parse_input1(&inp)));
    println!("Part 2: {}", solve(parse_input2(&inp)));
}
