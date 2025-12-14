use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

fn part_two(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut data = Vec::<Vec<String>>::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        data.push(line.split_whitespace().map(|s| s.to_string()).collect());
    }

    let operands = data.remove(data.len() - 1);
    let mut grand_total = 0;

    for (col_idx, operand) in operands.iter().enumerate() {
        let mut cols = 0;
        for line in data.iter() {
            cols = cols.max(line[col_idx].len());
        }
        let mut values_str = Vec::<String>::new();
        for line in data.iter() {
            let mut value_str = line[col_idx].clone();

            if operand == "+" {
                while value_str.len() < cols {
                    value_str.push(' ');
                }
            } else {
                while value_str.len() < cols {
                    value_str.insert(0, ' ');
                }
            }

            values_str.push(value_str);
        }

        let mut values = Vec::<usize>::new();
        for col in 0..cols {
            let mut new_value = String::new();
            for value in values_str.clone() {
                if value.as_bytes()[col] as char != ' ' {
                    new_value.push(value.as_bytes()[col] as char);
                }
            }
            let value = new_value.parse::<usize>()?;
            values.push(value);
        }

        let total: usize = match operand.as_str() {
            "*" => values.iter().product(),
            "+" => values.iter().sum(),
            _ => unreachable!(),
        };

        grand_total += total;
    }

    Ok(grand_total)
}

fn part_one(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut data = Vec::<Vec<String>>::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        data.push(line.split_whitespace().map(|s| s.to_string()).collect());
    }

    let operands = data.remove(data.len() - 1);
    let mut grand_total = 0;

    for (col_idx, operand) in operands.iter().enumerate() {
        let mut total = 0;
        for (row_idx, line) in data.iter().enumerate() {
            let value = line[col_idx].parse::<usize>()?;
            total = if row_idx == 0 {
                value
            } else {
                match operand.as_str() {
                    "*" => total * value,
                    "+" => total + value,
                    _ => unreachable!(),
                }
            };
        }
        grand_total += total;
    }

    Ok(grand_total)
}

fn main() -> Result<()> {
    println!("Part one - sample: {}", part_one(SAMPLE)?);
    println!("Part one - input: {}", part_one(INPUT)?);

    println!("Part two - sample: {}", part_two(SAMPLE)?);
    println!("Part two - input: {}", part_two(INPUT)?);
    Ok(())
}
