use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn Error>>;

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

fn part_two(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let final_len = 12;
    let mut total_output = 0;

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut id: usize = 0;
        let mut last_idx = 0;
        for n in 0..final_len {
            let remaining = final_len - n;
            let window = &digits[last_idx..digits.len() - remaining + 1];
            let mut idx = 0;
            let mut max = 0;
            for (i, digit) in window.iter().enumerate() {
                if *digit > max {
                    max = *digit;
                    idx = i;
                }
            }
            id = id * 10 + max as usize;
            last_idx += idx + 1;
        }
        total_output += id;
    }

    Ok(total_output)
}

fn part_one(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut total_output = 0;

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut m = 0;
        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                let double_batteries = digits[i] * 10 + digits[j];
                m = m.max(double_batteries);
            }
        }

        total_output += m;
    }

    Ok(total_output as usize)
}

fn main() -> Result<()> {
    println!("Part one - sample: {}", part_one(SAMPLE)?);
    println!("Part one - input: {}", part_one(INPUT)?);

    println!("Part two - sample: {}", part_two(SAMPLE)?);
    println!("Part two - input: {}", part_two(INPUT)?);
    Ok(())
}
