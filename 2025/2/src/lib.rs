/// Parse ranges in the format `19391-47353` and return a tuple
pub fn parse_range(text: &str) -> Option<(usize, usize)> {
    text.split_once('-').and_then(|(s, e)| {
        let start: Option<usize> = s.parse().ok();
        let end: Option<usize> = e.parse().ok();
        if let Some(start) = start
            && let Some(end) = end
        {
            return Some((start, end));
        }
        None
    })
}

/// Return true if the given number is made only of some sequence of digits repeated twice, e.g. 1010
pub fn sequence_is_repeated_twice(mut number: usize) -> bool {
    if number <= 10 {
        return false;
    }
    let mut digits = vec![];
    while number != 0 {
        digits.push((number % 10) as u8);
        number /= 10;
    }
    if digits.len() % 2 == 0 {
        return digits[0..digits.len() / 2] == digits[digits.len() / 2..];
    }

    false
}

/// Return true if the given number is made only of some sequence of digits repeated *at least* twice, e.g. 123123123
pub fn sequence_is_repeated(mut number: usize) -> bool {
    if number <= 10 {
        return false;
    }
    let mut digits = vec![];
    while number != 0 {
        digits.push((number % 10) as u8);
        number /= 10;
    }
    let mut n = digits.len() / 2;
    while n > 0 {
        let mut chunks = digits.chunks(n);
        let first_chunk = chunks.next().unwrap();
        if chunks.all(|c| c == first_chunk) {
            return true;
        }
        n -= 1;
    }

    false
}

fn sum_ranges<T: IntoIterator<Item = (usize, usize)>, F: Fn(usize) -> bool>(
    ranges: T,
    f: F,
) -> usize {
    let mut sum = 0;
    for (start, end) in ranges.into_iter() {
        sum += (start..=end).filter(|n| f(*n)).sum::<usize>();
    }
    sum
}

/// Return the sum of all numbers in the given ranges where `sequence_is_repeated_twice(n)` is `true`
pub fn sum_numbers_with_sequences_repeated_twice<T: IntoIterator<Item = (usize, usize)>>(
    ranges: T,
) -> usize {
    sum_ranges(ranges, sequence_is_repeated_twice)
}

/// Return the sum of all numbers in the given ranges where `sequence_is_repeated(n)` is `true`
pub fn sum_numbers_with_sequences_repeated<T: IntoIterator<Item = (usize, usize)>>(
    ranges: T,
) -> usize {
    sum_ranges(ranges, sequence_is_repeated)
}
