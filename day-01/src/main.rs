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

    let mut counter: usize = 0;
    let mut cursor: isize = 50;

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }

        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<isize>()?;

        for _ in 0..distance {
            match direction {
                "L" => {
                    cursor = (cursor - 1).rem_euclid(100);
                }
                "R" => {
                    cursor = (cursor + 1).rem_euclid(100);
                }
                _ => unreachable!(),
            }

            if cursor == 0 {
                counter += 1;
            }
        }
    }

    Ok(counter)
}

fn part_one(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut cursor: isize = 50;

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }

        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<isize>()?;

        cursor = match direction {
            "L" => (cursor - distance).rem_euclid(100),
            "R" => (cursor + distance).rem_euclid(100),
            _ => unreachable!(),
        };

        if cursor == 0 {
            counter += 1;
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
