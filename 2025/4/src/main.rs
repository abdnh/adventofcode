use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use printing_department::PaperGrid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn line_to_row<T: AsRef<str>>(line: T) -> Vec<bool> {
    line.as_ref().chars().map(|c| c == '@').collect()
}

fn main() -> Result<()> {
    let buffer = BufReader::new(File::open("./input.txt")?);
    let mut lines = buffer.lines();
    let first_line = lines.next().ok_or("empty file")??;
    let mut grid = PaperGrid::new(first_line.len());
    grid.add_row(line_to_row(first_line));
    for line in lines {
        grid.add_row(line_to_row(line?));
    }
    println!(
        "Initial accessible paper rolls: {:}",
        grid.get_accessible_rolls().len()
    );
    let removed = grid.remove_accessible_rolls();
    println!("Total removed paper rolls: {:}", removed);

    Ok(())
}
