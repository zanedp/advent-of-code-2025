fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = input.replace("\r\n", "\n"); // normalize line endings
    let mut sections = input.split("\n\n");
    let fresh_ranges = sections
        .next()
        .unwrap()
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut parts = line.split('-');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let prod_ids = sections.next().unwrap();
    let prod_ids = prod_ids
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    (fresh_ranges, prod_ids)
}

fn is_fresh(fresh_ranges: &[(u64, u64)], prod_id: u64) -> bool {
    fresh_ranges
        .iter()
        .any(|(start, end)| prod_id >= *start && prod_id <= *end)
}

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    let (fresh_ranges, prod_ids) = parse_input(input);
    let fresh_count = prod_ids
        .iter()
        .filter(|&prod_id| is_fresh(&fresh_ranges, *prod_id))
        .count();
    println!("Number of fresh products: {}", fresh_count);
}
