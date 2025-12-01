use std::fs;

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_input(inp: String) -> Vec<(Direction, i32)> {
    inp.lines()
        .map(|line| {
            let direction: Direction;
            if line[..1] == *"L" {
                direction = Direction::Left
            } else {
                direction = Direction::Right
            }
            (direction, line[1..].parse().unwrap())
        })
        .collect()
}

fn p1(inp: Vec<(Direction, i32)>) -> i32 {
    let mut position = 50;
    let mut count = 0;

    for (dir, dist) in inp {
        match dir {
            Direction::Left => position -= dist % 100,
            Direction::Right => position += dist % 100,
        };
        if position > 99 {
            position -= 100
        } else if position < 0 {
            position = 100 + position
        }
        if position == 0 {
            count += 1
        }
    }

    count
}

fn p2(inp: Vec<(Direction, i32)>) -> i32 {
    let mut position = 50;
    let mut count = 0;

    for (dir, dist) in inp {
        match dir {
            Direction::Left => position -= dist % 100,
            Direction::Right => position += dist % 100,
        };
        count += dist / 100;
        if position > 99 {
            count += 1;
            position -= 100;
        } else if position < 0 {
            if -dist % 100 != position {
                count += 1
            }
            position = 100 + position;
        } else if position == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let inp = parse_input(fs::read_to_string("input.txt").unwrap());

    println!("Part 1: {}", p1(inp.clone()));
    println!("Part 2: {}", p2(inp));
}
