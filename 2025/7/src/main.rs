use std::{error::Error, fs::File, io::BufReader};

use laboratories::{count_beam_splits_and_timelines, parse_grid};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let buffer = BufReader::new(File::open("./input.txt")?);
    let grid = parse_grid(buffer)?;
    let grid_copy = {
        let mut g = grid.clone();
        let mut v = vec![];
        v.extend_from_slice(grid.cells.as_slice());
        g.cells = v;

        g
    };
    let (split_count, timeline_count) = count_beam_splits_and_timelines(&grid_copy);
    println!("Split count: {split_count}\nTimeline count: {timeline_count}");

    Ok(())
}
