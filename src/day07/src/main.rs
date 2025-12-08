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

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
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
}
