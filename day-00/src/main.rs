use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn reader<P: AsRef<Path>>(path: P) -> BufReader<File> {
    let file = File::open(path).expect("couldn't open file");
    BufReader::new(file)
}

fn main() {
    println!("Hello, world!");
}
