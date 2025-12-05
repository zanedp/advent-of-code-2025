use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, i32)> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| (&line[0..1], &line[1..]))
        .map(|(dir_str, dist_str)| (dir_str.parse().unwrap(), dist_str.parse().unwrap()))
}

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    let instructions = parse_input(input);
    let mut cur_pos = 50;
    let mut zero_counts = 0;
    for (dir, dist) in instructions {
        let new_pos = match dir {
            Direction::Left => (cur_pos - dist).rem_euclid(100),
            Direction::Right => (cur_pos + dist).rem_euclid(100),
        };
        if new_pos == 0 {
            zero_counts += 1;
        }
        cur_pos = new_pos;
    }
    println!("Final position: {}", cur_pos);
    println!("Zero counts: {}", zero_counts);

    let instructions = parse_input(input);
    let mut cur_pos = 50i32;
    let mut zero_counts = 0;
    for (dir, dist) in instructions {
        let addend = match dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        for _ in 0..dist {
            cur_pos += addend;
            cur_pos = cur_pos.rem_euclid(100);
            if cur_pos == 0 {
                zero_counts += 1;
            }
        }
    }
    println!("Final position: {}", cur_pos);
    println!("Zero counts: {}", zero_counts);
}
