use lobby::{find_maximum_joltage_of_twelve_digits, find_maximum_joltage_of_two_digits};

#[test]
fn test_maximum_joltage() {
    assert_eq!(find_maximum_joltage_of_two_digits(&[]), 0);
    assert_eq!(
        find_maximum_joltage_of_two_digits(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
        98
    );
    assert_eq!(
        find_maximum_joltage_of_two_digits(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
        89
    );
    assert_eq!(
        find_maximum_joltage_of_two_digits(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
        78
    );
    assert_eq!(
        find_maximum_joltage_of_two_digits(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        92
    );

    assert_eq!(find_maximum_joltage_of_twelve_digits(&[]), 0);
    assert_eq!(
        find_maximum_joltage_of_twelve_digits(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
        987654321111
    );
    assert_eq!(
        find_maximum_joltage_of_twelve_digits(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
        811111111119
    );
    assert_eq!(
        find_maximum_joltage_of_twelve_digits(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
        434234234278
    );
    assert_eq!(
        find_maximum_joltage_of_twelve_digits(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        888911112111
    );
}
