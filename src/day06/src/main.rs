use core::panic;
use std::str::FromStr;

struct CephalopodMath {
    operands: Vec<u64>,
    operations: Vec<Operator>,
    num_columns: usize,
}

impl CephalopodMath {
    pub fn iter_for_column(&self, column_index: usize) -> impl Iterator<Item = u64> + '_ {
        self.operands
            .iter()
            .skip(column_index)
            .step_by(self.num_columns)
            .copied()
    }

    pub fn operation_for_column(&self, column_index: usize) -> Operator {
        self.operations[column_index]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> CephalopodMath {
    let mut math = CephalopodMath {
        operands: Vec::new(),
        operations: Vec::new(),
        num_columns: 0,
    };

    for line in input.lines() {
        if let Ok(numbers) = line
            .split_whitespace()
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>()
        {
            math.num_columns = numbers.len();
            math.operands.extend(numbers);
        } else if let Ok(operations) = line
            .split_whitespace()
            .map(Operator::from_str)
            .collect::<Result<Vec<_>, _>>()
        {
            math.operations = operations;
        } else {
            panic!("Invalid input: {line}");
        }
    }

    math
}

fn main() {
    let input = include_str!("sample-input.txt");
    // let input = include_str!("input.txt");
    println!("--- Part 1 ---");
    let math = parse_input(input);
    let mut grand_total = 0;
    for column_index in 0..math.num_columns {
        let column_values = math.iter_for_column(column_index);
        let operation = math.operation_for_column(column_index);
        let result: u64 = match operation {
            Operator::Add => column_values.sum(),
            Operator::Multiply => column_values.product(),
        };
        grand_total += result;
    }
    println!("Grand total: {}", grand_total);

    println!();
    println!("--- Part 2 ---");
    let chars: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().chain([' ', ' ', ' ', ' ']).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grand_total = 0;
    let mut column_num = 0;
    let mut cur_operator = None;
    let mut cur_operands = Vec::new();
    loop {
        let cur_column_chars = get_column(&chars, column_num);
        column_num += 1;
        if cur_column_chars.is_empty() || cur_column_chars.iter().all(|ch| ch.is_whitespace()) {
            // we got to the end of a math problem when there's no more data or all the characters are whitespace
            // if we have an operator and some operands, we need to apply it
            if cur_operator.is_none() && cur_operands.is_empty() {
                // we got to the end of the input without finding any more data
                break;
            }
            grand_total += match cur_operator {
                Some(Operator::Add) => cur_operands.iter().sum::<u64>(),
                Some(Operator::Multiply) => cur_operands.iter().product::<u64>(),
                None => {
                    panic!("No operator found");
                    // break;
                }
            };
            cur_operator = None;
            cur_operands.clear();
        } else {
            let cur_number_chars = &cur_column_chars[..cur_column_chars.len() - 1];
            let cur_number_as_string = String::from_iter(cur_number_chars);
            let cur_number_as_string = cur_number_as_string.trim();

            let operator_chars = &cur_column_chars[cur_column_chars.len() - 1..];
            let operator_as_string = String::from_iter(operator_chars);
            let operator_as_string = operator_as_string.trim();

            if let Ok(num) = cur_number_as_string.parse::<u64>() {
                cur_operands.push(num);
            } else {
                panic!(
                    "Invalid character in column {column_num}: {:?}",
                    cur_column_chars
                );
            }

            if operator_as_string.is_empty() {
                // most columns don't have operators at the end, so we just ignore them
            } else if let Ok(op) = operator_as_string.parse::<Operator>() {
                cur_operator = Some(op);
            } else {
                panic!(
                    "Invalid character in column {column_num}: {:?}",
                    cur_column_chars
                );
            }
        }
        if cur_column_chars.is_empty() {
            break;
        }
    }
    println!("Grand total: {}", grand_total);
}

fn get_column(chars: &Vec<Vec<char>>, column_index: usize) -> Vec<char> {
    let mut column = Vec::new();
    for row in chars {
        if let Some(&c) = row.get(column_index) {
            column.push(c);
        }
    }
    column
}
