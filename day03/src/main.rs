use std::fs;

fn parse_input(inp: String) -> Vec<Vec<u8>> {
    inp.lines()
        .map(|line| line.as_bytes().iter().map(|v| v - '0' as u8).collect())
        .collect()
}

fn solve(inp: Vec<Vec<u8>>, iterations: u32) -> u64 {
    let mut sum: u64 = 0;

    for bank in inp {
        let mut m: u64 = 10_u64.pow(iterations - 1);
        let mut skip: usize = 0;

        for i in (0..iterations as usize).rev() {
            let mut b = bank.iter().take(bank.len() - i).skip(skip);
            let max = b.clone().max().unwrap();
            skip += b.position(|v| v == max).unwrap() + 1;
            sum += *max as u64 * m;
            m /= 10;
        }
    }

    sum
}

fn p1(inp: Vec<Vec<u8>>) -> u64 {
    solve(inp, 2)
}

fn p2(inp: Vec<Vec<u8>>) -> u64 {
    solve(inp, 12)
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(inp.clone()));
    println!("Part 2: {}", p2(inp));
}
