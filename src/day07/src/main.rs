fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn beam_through_next_row(cur_row: &[char], next_row: &[char]) -> (Vec<char>, u64) {
    let mut beamed_next_row = next_row.to_vec();
    let mut split_count = 0;
    for (i, &c) in cur_row.iter().enumerate() {
        if c == 'S' || c == '|' {
            if next_row[i] == '^' {
                beamed_next_row[i - 1] = '|';
                beamed_next_row[i + 1] = '|';
                split_count += 1;
            } else if next_row[i] == '.' {
                beamed_next_row[i] = '|';
            }
        }
    }
    (beamed_next_row, split_count)
}

fn print_row(row: &[char]) {
    println!("{}", String::from_iter(row));
}

fn quantum_tunnel_through_next_row(cur_row: &[u64], next_row: &[char]) -> Vec<u64> {
    let mut next_row_paths = vec![0; next_row.len()];
    for (i, &c) in cur_row.iter().enumerate() {
        if next_row[i] == '^' {
            next_row_paths[i - 1] += c;
            next_row_paths[i + 1] += c;
        } else if next_row[i] == '.' {
            next_row_paths[i] += c;
        }
    }
    next_row_paths
}

fn start(first_row: &[char]) -> usize {
    for (i, &c) in first_row.iter().enumerate() {
        if c == 'S' {
            return i;
        }
    }
    panic!("No starting point found");
}

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    let input_grid = parse_input(input);
    let mut split_count = 0;
    let mut cur_row = input_grid[0].clone();
    print_row(&cur_row);
    for next_row in input_grid.iter().skip(1) {
        let (beamed_next_row, next_row_split_count) = beam_through_next_row(&cur_row, next_row);
        print_row(&beamed_next_row);
        split_count += next_row_split_count;
        cur_row = beamed_next_row;
    }
    println!("Split count: {}", split_count);
    println!();
    println!("--- Part 2 ---");
    let input_grid = parse_input(input);
    let mut cur_row = vec![0u64; input_grid[0].len()];
    cur_row[start(&input_grid[0])] = 1;
    for next_row in input_grid.iter().skip(1) {
        cur_row = quantum_tunnel_through_next_row(&cur_row, next_row);
    }
    println!("Total paths: {}", cur_row.iter().sum::<u64>());
}
