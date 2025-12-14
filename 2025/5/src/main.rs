use cafeteria::{Cafeteria, parse_range};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut cafeteria = Cafeteria::new();
    let buffer = BufReader::new(File::open("./input.txt")?);
    let mut parsing_ranges = true;
    let mut fresh_count = 0;
    for line in buffer.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }
        if parsing_ranges {
            let range = parse_range(line).ok_or("invalid range")?;
            cafeteria.add_range(range);
        } else {
            let id: usize = line.parse()?;
            if cafeteria.is_fresh(id) {
                fresh_count += 1;
            }
        }
    }
    println!("Fresh ID count: {fresh_count}");
    println!("Range ID count: {}", cafeteria.count_range_ids());

    Ok(())
}
