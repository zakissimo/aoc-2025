use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn Error>>;

const SAMPLE: &str = "sample.txt";
const INPUT: &str = "input.txt";

fn is_inbound(x: isize, row_len: usize, y: isize, col_len: usize) -> bool {
    (x >= 0 && x < row_len as isize) && (y >= 0 && y < col_len as isize)
}

fn remove_rolls(grid: &mut [Vec<char>]) -> usize {
    let dirs = [-1, 0, 1];
    let mut changes = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c != '@' {
                continue;
            }

            let mut count = 0;
            for dx in dirs {
                for dy in dirs {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let dx = x as isize + dx as isize;
                    let dy = y as isize + dy as isize;

                    let row_len = grid[y].len();
                    let grid_len = grid.len();
                    if is_inbound(dx, row_len, dy, grid_len)
                        && grid[dy as usize][dx as usize] == '@'
                    {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                changes += 1;
                grid[y][x] = '.';
            }
        }
    }

    changes
}

fn part_two(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        grid.push(line.chars().collect());
    }

    let mut total_removed = 0;

    loop {
        let removed = remove_rolls(&mut grid);
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }

    Ok(total_removed)
}

fn part_one(input: &str) -> Result<usize> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        grid.push(line.chars().collect());
    }

    let dirs = [-1, 0, 1];
    let mut accessible = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '@' {
                continue;
            }

            let mut count = 0;
            for dx in dirs {
                for dy in dirs {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let dx = x as isize + dx as isize;
                    let dy = y as isize + dy as isize;
                    if is_inbound(dx, row.len(), dy, grid.len())
                        && grid[dy as usize][dx as usize] == '@'
                    {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                accessible += 1;
            }
        }
    }

    Ok(accessible)
}

fn main() -> Result<()> {
    println!("Part one - sample: {}", part_one(SAMPLE)?);
    println!("Part one - input: {}", part_one(INPUT)?);

    println!("Part two - sample: {}", part_two(SAMPLE)?);
    println!("Part two - input: {}", part_two(INPUT)?);
    Ok(())
}
