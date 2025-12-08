use std::ops::RangeInclusive;
use std::{error::Error, fs::read_to_string, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

fn part_two(input: &str) -> Result<usize> {
    let raw = read_to_string(input)?;
    let (raw_ranges, _) = raw
        .split_once("\n\n")
        .expect("AOC lied on the input format");

    let mut ranges: Vec<RangeInclusive<usize>> = raw_ranges
        .split_whitespace()
        .filter_map(|line| {
            let (start, end) = line.split_once("-")?;
            Some(start.parse::<usize>().ok()?..=end.parse::<usize>().ok()?)
        })
        .collect();

    if ranges.is_empty() {
        return Ok(0);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut count = 0;
    let mut curr = ranges[0].clone();

    for range in &ranges[1..] {
        if *range.start() <= *curr.end() {
            curr = RangeInclusive::new(*curr.start(), *curr.end().max(range.end()));
        } else {
            count += *curr.end() - *curr.start() + 1;
            curr = range.clone();
        }
    }

    count += *curr.end() - *curr.start() + 1;

    Ok(count)
}

fn part_one(input: &str) -> Result<usize> {
    let raw = read_to_string(input)?;
    let (raw_ranges, raw_ids) = raw
        .split_once("\n\n")
        .expect("AOC lied on the input format");

    let ranges: Vec<RangeInclusive<usize>> = raw_ranges
        .split_whitespace()
        .filter_map(|line| {
            let (start, end) = line.split_once("-")?;
            Some(start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
        })
        .collect();

    let ids: Vec<usize> = raw_ids
        .split_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut counter = 0;

    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                counter += 1;
                break;
            }
        }
    }

    Ok(counter)
}

fn main() -> Result<()> {
    println!("Part one - sample: {}", part_one(SAMPLE)?);
    println!("Part one - input: {}", part_one(INPUT)?);

    println!("Part two - sample: {}", part_two(SAMPLE)?);
    println!("Part two - input: {}", part_two(INPUT)?);
    Ok(())
}
