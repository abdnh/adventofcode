use std::ops::Add;
use std::ops::Mul;

use trash_compactor::apply_operation;

#[test]
fn test_operations() {
    assert_eq!(apply_operation(&[123, 45, 6], usize::mul), 33210);
    assert_eq!(apply_operation(&[328, 64, 98], usize::add), 490);
}
