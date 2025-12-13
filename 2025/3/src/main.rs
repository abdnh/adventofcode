use lobby::{find_maximum_joltage_of_twelve_digits, find_maximum_joltage_of_two_digits};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let buffer = BufReader::new(File::open("./input.txt")?);
    let mut sum2 = 0;
    let mut sum12 = 0;
    for line in buffer.lines() {
        let bank: Vec<u8> = line?
            .chars()
            .filter_map(|c| c.is_ascii_digit().then_some(c as u8 - b'0'))
            .collect();
        sum2 += find_maximum_joltage_of_two_digits(bank.as_slice());
        sum12 += find_maximum_joltage_of_twelve_digits(bank.as_slice());
    }

    println!("sum2={sum2} sum12={sum12}");

    Ok(())
}
