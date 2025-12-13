use gift_shop::{parse_range, sum_numbers_with_sequences_repeated};
use std::{
    error::Error,
    fs::File,
    io::{self},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let text = io::read_to_string(File::open("./input.txt")?)?;
    let text = text.trim();
    let mut ranges = vec![];
    for range in text.split(',') {
        ranges.push(parse_range(range).ok_or(format!("Invalid range: {range}"))?);
    }

    let sum = sum_numbers_with_sequences_repeated(ranges);
    println!("{sum}");

    Ok(())
}
