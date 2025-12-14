use cafeteria::{Cafeteria, parse_range};

fn assert_fresh_id_count(
    ranges: &[(usize, usize)],
    ids_to_check: &[usize],
    expected_fresh_ids: &[usize],
) {
    let mut cafeteria = Cafeteria::new();
    for range in ranges {
        cafeteria.add_range(*range);
    }
    let mut fresh_ids = vec![];
    for id in ids_to_check.iter().copied() {
        if cafeteria.is_fresh(id) {
            fresh_ids.push(id);
        }
    }
    assert_eq!(fresh_ids, expected_fresh_ids);
}

fn assert_range_id_count(ranges: &[(usize, usize)], expected_count: usize) {
    let mut cafeteria = Cafeteria::new();
    for range in ranges {
        cafeteria.add_range(*range);
    }
    assert_eq!(cafeteria.count_range_ids(), expected_count);
}

#[test]
fn test_range_parsing() {
    assert_eq!(parse_range("19391-47353"), Some((19391, 47353)));
    assert_eq!(parse_range("-23-44"), None);
    assert_eq!(parse_range("foo-bar"), None);
    assert_eq!(parse_range("23-foo"), None);
    assert_eq!(parse_range("foo-23"), None);
}

#[test]
fn test_fresh_id_counting() {
    assert_fresh_id_count(
        &[(3, 5), (10, 14), (16, 20), (12, 18)],
        &[1, 5, 8, 11, 17, 32],
        &[5, 11, 17],
    );
}

#[test]
fn test_range_id_counting() {
    assert_range_id_count(&[(3, 5), (10, 14), (16, 20), (12, 18)], 14);
    assert_range_id_count(&[(3, 5), (10, 14), (16, 20), (12, 13)], 13);
    assert_range_id_count(&[(3, 5), (10, 14), (11, 15), (16, 20), (12, 13)], 14);
    assert_range_id_count(&[(3, 5), (10, 14), (14, 14), (16, 20), (12, 13)], 13);
}
