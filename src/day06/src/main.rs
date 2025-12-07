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
}
