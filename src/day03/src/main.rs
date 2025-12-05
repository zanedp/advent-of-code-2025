fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

fn find_best_joltage(cell_values: &[u64], num_digits: usize) -> u64 {
    let mut search_start_pos = 0;
    let mut best_joltage = 0u64;
    for i in 0..num_digits {
        let num_digits_remaining = num_digits - i - 1;
        let haystack_digits =
            &cell_values[search_start_pos..cell_values.len() - num_digits_remaining];
        let Some((pos, new_digit)) = haystack_digits
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_pos, &val)| val)
        else {
            panic!("No max found");
        };
        best_joltage = best_joltage * 10 + new_digit;
        search_start_pos += pos + 1;
    }
    best_joltage
}

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    // indexed by [bank][cell]
    let battery_banks = parse_input(input);
    let total_joltage = battery_banks
        .iter()
        .map(|bank| find_best_joltage(bank, 2))
        .sum::<u64>();
    println!("Total joltage: {}", total_joltage);

    println!();
    println!("--- Part 2 ---");
    let battery_banks = parse_input(input);
    let total_joltage = battery_banks
        .iter()
        .map(|bank| find_best_joltage(bank, 12))
        .sum::<u64>();
    println!("Total joltage: {}", total_joltage);
}
