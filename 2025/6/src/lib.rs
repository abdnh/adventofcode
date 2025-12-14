pub fn apply_operation<F: Fn(usize, usize) -> usize>(numbers: &[usize], op: F) -> usize {
    let mut result = numbers[0];
    for number in numbers[1..].iter() {
        result = op(result, *number);
    }

    result
}
