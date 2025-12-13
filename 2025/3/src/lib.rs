fn find_maximum_joltage(bank: &[u8], n: u32) -> usize {
    if bank.is_empty() {
        return 0;
    }

    let mut last_digit_pos: Option<usize> = None;
    let mut sum: usize = 0;
    for i in 1..=n {
        let mut max_idx = 0;
        let mut max_joltage = 0;
        let start_pos = if let Some(last_pos) = last_digit_pos {
            last_pos + 1
        } else {
            0
        };
        for (idx, joltage) in bank[start_pos..bank.len() - (n - i) as usize]
            .iter()
            .enumerate()
        {
            if *joltage > max_joltage {
                max_idx = start_pos + idx;
                max_joltage = *joltage;
            }
        }
        last_digit_pos = Some(max_idx);
        sum += (max_joltage as usize) * 10usize.pow(n - i);
    }

    sum
}

pub fn find_maximum_joltage_of_two_digits(bank: &[u8]) -> usize {
    // if bank.is_empty() {
    //     return 0;
    // }

    // let mut max_idx = 0;
    // let mut max_joltage: u16 = 0;
    // for (idx, joltage) in bank[0..bank.len() - 1].iter().enumerate() {
    //     if *joltage as u16 > max_joltage {
    //         max_idx = idx;
    //         max_joltage = *joltage as u16;
    //     }
    // }
    // let mut next_max_joltage: u16 = 0;
    // for joltage in bank[max_idx + 1..].iter() {
    //     if *joltage as u16 > next_max_joltage {
    //         next_max_joltage = *joltage as u16;
    //     }
    // }

    // return (max_joltage) * 10 + next_max_joltage;

    find_maximum_joltage(bank, 2)
}

pub fn find_maximum_joltage_of_twelve_digits(bank: &[u8]) -> usize {
    find_maximum_joltage(bank, 12)
}
