use std::ops::Add;
use std::ops::Mul;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
use trash_compactor::apply_operation;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_operation(op: char) -> Option<Box<dyn Fn(usize, usize) -> usize>> {
    match op {
        '+' => Some(Box::new(usize::add)),
        '*' => Some(Box::new(usize::mul)),
        _ => None,
    }
}

struct DigitInfo {
    chr: char,
}

fn main() -> Result<()> {
    let mut operations: Vec<char> = vec![];
    let mut n = 0;
    let mut grid: Vec<DigitInfo> = vec![];
    let buffer = BufReader::new(File::open("./input.txt")?);
    for (line_index, line) in buffer.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        for chr in line.chars() {
            if chr.is_ascii_digit() {
            } else if !chr.is_whitespace() {
                operations.push(chr);
            }
            if line_index == 0 {
                n += 1;
            }
            grid.push(DigitInfo { chr });
        }
    }
    let mut sum: usize = 0;
    let mut current_operands: Vec<usize> = vec![];
    let mut current_operation_idx: usize = 0;
    for (i, digit_info) in grid[0..n].iter().enumerate() {
        let mut digits: Vec<u8> = vec![];
        let all_columns_empty = grid[i..grid.len()]
            .iter()
            .step_by(n)
            .all(|d| d.chr.is_ascii_whitespace());
        if digit_info.chr.is_ascii_digit() {
            digits.push(digit_info.chr as u8);
        }
        for digit_info2 in grid[i + n..grid.len()].iter().step_by(n) {
            if digit_info2.chr.is_ascii_digit() {
                digits.push(digit_info2.chr as u8);
            }
        }
        if !digits.is_empty() {
            let num: usize = String::from_utf8(digits)?.parse()?;
            current_operands.push(num);
        }
        if !current_operands.is_empty() && (i == n - 1 || all_columns_empty) {
            let operation = get_operation(operations[current_operation_idx]).ok_or(format!(
                "invalid operation: {}",
                operations[current_operation_idx]
            ))?;
            sum += apply_operation(&current_operands, operation);
            current_operands.clear();
            current_operation_idx += 1;
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
