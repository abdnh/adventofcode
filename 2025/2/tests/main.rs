use gift_shop::{
    parse_range, sequence_is_repeated, sequence_is_repeated_twice,
    sum_numbers_with_sequences_repeated, sum_numbers_with_sequences_repeated_twice,
};

#[test]
fn test_range_parsing() {
    assert_eq!(parse_range("19391-47353"), Some((19391, 47353)));
    assert_eq!(parse_range("-23-44"), None);
    assert_eq!(parse_range("foo-bar"), None);
    assert_eq!(parse_range("23-foo"), None);
    assert_eq!(parse_range("foo-23"), None);
}

#[test]
fn test_sequence_detection() {
    assert!(!sequence_is_repeated_twice(0));
    assert!(!sequence_is_repeated_twice(2));
    assert!(!sequence_is_repeated_twice(15));
    assert!(sequence_is_repeated_twice(11));
    assert!(sequence_is_repeated_twice(2020));
    assert!(sequence_is_repeated_twice(38593859));

    assert!(!sequence_is_repeated(0));
    assert!(!sequence_is_repeated(2));
    assert!(!sequence_is_repeated(15));
    assert!(sequence_is_repeated(11));
    assert!(sequence_is_repeated(2020));
    assert!(sequence_is_repeated(38593859));
    assert!(sequence_is_repeated(1188511885));
    assert!(sequence_is_repeated(565656));
    assert!(sequence_is_repeated(824824824));
    assert!(sequence_is_repeated(2121212121));
}

#[test]
fn test_summation() {
    assert_eq!(sum_numbers_with_sequences_repeated_twice([(11, 22)]), 33);
    assert_eq!(sum_numbers_with_sequences_repeated_twice([(95, 115)]), 99);
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(998, 1012)]),
        1010
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(1188511880, 1188511890)]),
        1188511885
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(222220, 222224)]),
        222222
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(1698522, 1698528)]),
        0
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(446443, 446449)]),
        446446
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated_twice([(38593856, 38593862)]),
        38593859
    );

    assert_eq!(sum_numbers_with_sequences_repeated([(11, 22)]), 33);
    assert_eq!(sum_numbers_with_sequences_repeated([(95, 115)]), 210);
    assert_eq!(sum_numbers_with_sequences_repeated([(998, 1012)]), 2009);
    assert_eq!(
        sum_numbers_with_sequences_repeated([(1188511880, 1188511890)]),
        1188511885
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated([(222220, 222224)]),
        222222
    );
    assert_eq!(sum_numbers_with_sequences_repeated([(1698522, 1698528)]), 0);
    assert_eq!(
        sum_numbers_with_sequences_repeated([(446443, 446449)]),
        446446
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated([(38593856, 38593862)]),
        38593859
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated([(565653, 565659)]),
        565656
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated([(824824821, 824824827)]),
        824824824
    );
    assert_eq!(
        sum_numbers_with_sequences_repeated([(2121212118, 2121212124)]),
        2121212121
    );
}
