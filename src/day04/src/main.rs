fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect())
        .collect()
}

fn count_neighboring_paper(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (dr, dc) in deltas.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
            && grid[new_row as usize][new_col as usize] == '@'
        {
            count += 1;
        }
    }
    count
}

fn main() {
    // let input = include_str!("sample-input.txt");
    let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    let grid = parse_input(input);
    let mut num_accessible = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' {
                // don't need to move anything out of empty squares.
                continue;
            }
            let count = count_neighboring_paper(&grid, row, col);
            if count < 4 {
                // println!("({}, {}) has {} neighboring paper pieces", row, col, count);
                num_accessible += 1;
            }
        }
    }
    println!("Number of accessible squares: {}", num_accessible);
    println!();

    println!("--- Part 2 ---");
    let mut grid = parse_input(input);
    let mut total_removed = 0;
    loop {
        let mut new_grid = grid.clone();
        let mut num_removed_this_round = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '.' || grid[row][col] == 'x' {
                    // don't need to move anything out of empty squares.
                    continue;
                }
                let count = count_neighboring_paper(&grid, row, col);
                if count < 4 {
                    // println!("({}, {}) has {} neighboring paper pieces", row, col, count);
                    new_grid[row][col] = 'x';
                    num_removed_this_round += 1;
                }
            }
        }
        if num_removed_this_round == 0 {
            // can't remove any more
            break;
        }
        // println!("Removed {} pieces this round", num_removed_this_round);
        total_removed += num_removed_this_round;
        grid = new_grid;
    }
    println!("Number of removed rolls: {}", total_removed);
}
