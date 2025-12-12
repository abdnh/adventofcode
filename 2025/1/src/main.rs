use std::{
    fs::File,
    io::{self, BufRead},
};

use secret_entrance::{SecretSafe, SecurityMethod, UnlockResult};

fn main() -> UnlockResult<()> {
    let mut safe = SecretSafe::default();
    let input: Vec<String> = io::BufReader::new(File::open("./input.txt")?)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?;
    let result = safe.unlock(input.iter().map(|s| s.as_str()), SecurityMethod::Any)?;
    println!("end position: {}\nsolution: {}", safe.end, result);
    Ok(())
}
