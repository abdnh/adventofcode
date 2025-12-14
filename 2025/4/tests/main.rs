use printing_department::PaperGrid;

fn assert_accessible_paper_roll_count(n: usize, paper_roll_count: usize, data: &[u8]) {
    let mut grid = PaperGrid::new(n);
    for chunk in data.chunks(n) {
        let row: Vec<bool> = chunk.iter().map(|i| i > &0).collect();
        grid.add_row(row);
    }
    assert_eq!(grid.get_accessible_rolls().len(), paper_roll_count);
}

#[test]
fn test_paper_roll_counting() {
    assert_accessible_paper_roll_count(10, 6, &[0, 0, 1, 1, 0, 1, 1, 1, 1, 0]);

    assert_accessible_paper_roll_count(
        10,
        12,
        &[
            0, 0, 1, 1, 0, 1, 1, 1, 1, 0, //
            1, 1, 1, 0, 1, 0, 1, 0, 1, 1, //
        ],
    );

    assert_accessible_paper_roll_count(
        10,
        11,
        &[
            0, 0, 1, 1, 0, 1, 1, 1, 1, 0, //
            1, 1, 1, 0, 1, 0, 1, 0, 1, 1, //
            1, 1, 1, 1, 1, 0, 1, 0, 1, 1, //
        ],
    );

    assert_accessible_paper_roll_count(
        10,
        13,
        &[
            0, 0, 1, 1, 0, 1, 1, 1, 1, 0, //
            1, 1, 1, 0, 1, 0, 1, 0, 1, 1, //
            1, 1, 1, 1, 1, 0, 1, 0, 1, 1, //
            1, 0, 1, 1, 1, 1, 0, 0, 1, 0, //
            1, 1, 0, 1, 1, 1, 1, 0, 1, 1, //
            0, 1, 1, 1, 1, 1, 1, 1, 0, 1, //
            0, 1, 0, 1, 0, 1, 0, 1, 1, 1, //
            1, 0, 1, 1, 1, 0, 1, 1, 1, 1, //
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0, //
            1, 0, 1, 0, 1, 1, 1, 0, 1, 0, //
        ],
    )
}
