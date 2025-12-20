use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::{
    fmt::{self},
    io::BufRead,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CellType {
    Beam,
    Empty,
    Splitter,
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            CellType::Empty => '.',
            CellType::Beam => '|',
            CellType::Splitter => '^',
        };
        write!(f, "{}", chr)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<CellType>,
    pub cols: usize,
    pub rows: usize,
}

#[derive(Debug)]
pub enum InputParseError {
    IoError(std::io::Error),
    InvalidCell(char),
}

impl fmt::Display for InputParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::InvalidCell(cell) => write!(f, "Invalid cell: {}", cell),
        }
    }
}

impl From<std::io::Error> for InputParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Error for InputParseError {}

pub fn parse_grid<T: BufRead>(buffer: T) -> Result<Grid, InputParseError> {
    let mut grid: Vec<CellType> = vec![];
    let mut cols = 0;
    let mut rows = 0;
    for line in buffer.lines() {
        let line = line?;
        let line = line.trim();
        if cols == 0 {
            cols = line.chars().count();
        }
        rows += 1;
        for chr in line.chars() {
            let cell_type = match chr {
                '.' => CellType::Empty,
                'S' => CellType::Beam,
                '^' => CellType::Splitter,
                _ => return Err(InputParseError::InvalidCell(chr)),
            };
            grid.push(cell_type);
        }
    }

    Ok(Grid {
        cells: grid,
        cols,
        rows,
    })
}

pub fn print_grid(grid: &Grid) {
    for j in 0..grid.rows {
        for i in 0..grid.cols {
            print!("{}", grid.cells[grid.cols * j + i]);
        }
        println!()
    }
    println!()
}

pub fn count_beam_splits_and_timelines(grid: &Grid) -> (usize, usize) {
    let rows = grid.rows;
    let cols = grid.cols;
    let mut pos_to_beams: HashMap<usize, usize> = HashMap::new();
    let mut split_count = 0;

    for i in 0..cols {
        if grid.cells[i] == CellType::Beam {
            pos_to_beams.insert(i, 1);
        }
    }

    for j in 1..rows {
        for i in 0..cols {
            match grid.cells[cols * j + i] {
                CellType::Beam => {}
                CellType::Splitter => {
                    if j == 0 {
                        continue;
                    }
                    let base_count = if let Some(count) = pos_to_beams.remove(&(cols * (j - 1) + i))
                    {
                        split_count += 1;
                        count
                    } else {
                        continue;
                    };
                    if i != 0 {
                        pos_to_beams
                            .entry(cols * j + i - 1)
                            .and_modify(|c| *c += base_count)
                            .or_insert(base_count);
                    }
                    if i != cols - 1 {
                        let additional_count =
                            pos_to_beams.remove(&(cols * (j - 1) + i + 1)).unwrap_or(0);
                        pos_to_beams.insert(cols * j + i + 1, base_count + additional_count);
                    }
                }
                CellType::Empty => {
                    if j != 0 && !pos_to_beams.contains_key(&(cols * j + i)) {
                        let beam_count =
                            if let Some(count) = pos_to_beams.remove(&(cols * (j - 1) + i)) {
                                count
                            } else {
                                continue;
                            };
                        pos_to_beams.insert(cols * j + i, beam_count);
                    }
                }
            }
        }
    }

    (split_count, pos_to_beams.values().sum())
}
