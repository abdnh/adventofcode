use core::fmt;
use std::io::{self};

#[derive(Debug)]
pub enum UnlockError {
    InvalidRotation,
    IoError(io::Error),
}

impl From<io::Error> for UnlockError {
    fn from(value: io::Error) -> Self {
        UnlockError::IoError(value)
    }
}

impl fmt::Display for UnlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e)?,
            Self::InvalidRotation => write!(f, "invalid rotation")?,
        }

        Ok(())
    }
}

pub type UnlockResult<T> = Result<T, UnlockError>;

#[derive(Debug)]
pub struct SecretSafe {
    start: i64,
    pub end: i64,
}

fn parse_rotation(rotation: &str) -> UnlockResult<i64> {
    let mut chars = rotation.chars();
    let sign: i64 = match chars.next() {
        Some('L') => -1,
        Some('R') => 1,
        _ => return Err(UnlockError::InvalidRotation),
    };
    let shift: u32 = chars
        .as_str()
        .parse()
        .map_err(|_| UnlockError::InvalidRotation)?;

    Ok(sign * i64::from(shift))
}

pub enum SecurityMethod {
    /// Counts number of times the dial points to zero at the end of each rotation
    End,
    /// Counts number of times the dial points to zero any time during a rotation
    Any,
}

impl SecretSafe {
    pub fn new(start: i64) -> Self {
        Self { start, end: start }
    }

    pub fn unlock<'a, T: IntoIterator<Item = &'a str>>(
        &mut self,
        rotations: T,
        method: SecurityMethod,
    ) -> UnlockResult<usize> {
        let mut zeroes: usize = 0;
        let mut current = self.start;
        for rotation in rotations.into_iter() {
            let shift = parse_rotation(rotation)?;
            let old_current = current;
            current = (current + shift).rem_euclid(100);
            match method {
                SecurityMethod::End => {
                    if current == 0 {
                        zeroes += 1;
                    }
                }
                SecurityMethod::Any => {
                    let factor = (shift / 100).abs() as usize;
                    zeroes += factor;
                    let shift_rem = shift - factor as i64 * 100 * shift.signum();
                    let sum = old_current + shift_rem;
                    if (sum <= 0 && old_current != 0) || sum >= 100 {
                        zeroes += 1;
                    }
                }
            }
        }
        self.end = current;

        Ok(zeroes)
    }
}

impl Default for SecretSafe {
    fn default() -> Self {
        Self::new(50)
    }
}
