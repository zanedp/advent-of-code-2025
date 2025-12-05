fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input
        .split(',')
        .map(str::trim)
        .map(|range_str| range_str.split_once('-').unwrap())
        .map(|(start_str, end_str)| (start_str.parse().unwrap(), end_str.parse().unwrap()))
}
fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    let ranges = parse_input(input);
    let mut invalid_count = 0;
    let mut sum_of_invalids = 0;
    for (start, end) in ranges {
        for cur_id in start..=end {
            let id_as_text = cur_id.to_string();
            let middle = id_as_text.len() / 2;
            let (first_half, second_half) = id_as_text.split_at(middle);
            if first_half == second_half {
                invalid_count += 1;
                sum_of_invalids += cur_id;
            }
        }
    }
    println!("Invalid count: {}", invalid_count);
    println!("Sum of invalids: {}", sum_of_invalids);

    println!("--- Part 2 ---");
    let ranges = parse_input(input);
    let mut invalid_count = 0;
    let mut sum_of_invalids = 0;
    for (start, end) in ranges {
        'ids_in_range: for cur_id in start..=end {
            let id_as_text = cur_id.to_string();
            let chars = id_as_text.chars().collect::<Vec<_>>();
            for n in 1..=chars.len() / 2 {
                let chunks_of_n = chars.chunks(n).map(String::from_iter).collect::<Vec<_>>();
                let first = &chunks_of_n[0];
                let all_equal = chunks_of_n.iter().all(|chunk| chunk == first);
                if all_equal {
                    invalid_count += 1;
                    sum_of_invalids += cur_id;
                    continue 'ids_in_range;
                }
            }
        }
    }
    println!("Invalid count: {}", invalid_count);
    println!("Sum of invalids: {}", sum_of_invalids);
}
