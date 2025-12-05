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
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let line = line.trim();
    let mut ids = 0;

    for range in line.split(',') {
        let Some((start, end)) = range.split_once('-') else {
            continue;
        };

        let start = start.parse::<usize>()?;
        let end = end.parse::<usize>()?;

        for id in start..=end {
            let size = id.ilog10() as usize + 1;
            let mut chunks = Vec::<usize>::new();

            let mut id_tmp = id;
            while id_tmp > 0 {
                chunks.push(id_tmp % 10);
                id_tmp /= 10;
            }

            let mid = size / 2;
            for n in 1usize..=mid {
                let mut chunk_compare = None;
                for (i, chunk) in chunks.chunks(n).enumerate() {
                    if let Some(c) = chunk_compare
                        && c != chunk
                    {
                        chunk_compare = None;
                        break;
                    };

                    if i == 0 {
                        chunk_compare = Some(chunk);
                    }
                }
                if chunk_compare.is_some() {
                    ids += id;
                    break;
                }
            }
        }
    }

    Ok(ids)
}

fn part_one(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let line = line.trim();
    let mut ids = 0;

    for range in line.split(',') {
        let Some((start, end)) = range.split_once('-') else {
            continue;
        };

        let start = start.parse::<usize>()?;
        let end = end.parse::<usize>()?;

        for id in start..=end {
            let size = id.ilog10() + 1;
            if size % 2 != 0 {
                continue;
            }
            let mid = size / 2;
            let p = 10usize.pow(mid);

            if id / p == id % p {
                ids += id;
            }
        }
    }

    Ok(ids)
}

fn main() -> Result<()> {
    println!("Part one - sample: {}", part_one(SAMPLE)?);
    println!("Part one - input: {}", part_one(INPUT)?);

    println!("Part two - sample: {}", part_two(SAMPLE)?);
    println!("Part two - input: {}", part_two(INPUT)?);
    Ok(())
}
