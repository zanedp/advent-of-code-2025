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
    println!();
    println!("--- Part 2 ---");
    let (fresh_ranges, _prod_ids) = parse_input(input);
    let mut fresh_ranges_sorted = fresh_ranges.clone();
    fresh_ranges_sorted.sort_by_key(|(start, _end)| *start);
    let fresh_ranges_sorted = fresh_ranges_sorted;
    let mut ranges_iter = fresh_ranges_sorted.iter();
    let mut detangled_ranges = Vec::new();
    let Some(&(mut a_start, mut a_end)) = ranges_iter.next() else {
        panic!("No ranges found");
    };
    loop {
        let Some(&(b_start, b_end)) = ranges_iter.next() else {
            detangled_ranges.push((a_start, a_end));
            break;
        };

        if b_start > a_end {
            // no overlap. Take a as-is
            detangled_ranges.push((a_start, a_end));
            a_start = b_start;
            a_end = b_end;
        } else {
            // overlap, so we need to extend a to include b
            a_end = a_end.max(b_end);
        }
    }
    // println!("Detangled ranges: {:?}", detangled_ranges);
    let num_fresh_ids = detangled_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();
    println!("Number of fresh IDs: {}", num_fresh_ids);
}
