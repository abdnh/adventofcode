use secret_entrance::{SecretSafe, SecurityMethod, UnlockResult};

type TestResult = UnlockResult<()>;

fn test_rotations(input: &[&str], solution: usize, end: i64, method: SecurityMethod) -> TestResult {
    let mut safe = SecretSafe::default();
    let result = safe.unlock(input.iter().map(|s| *s), method)?;
    assert_eq!(result, solution);
    assert_eq!(safe.end, end);

    Ok(())
}

fn test_rotations_end(input: &[&str], solution: usize, end: i64) -> TestResult {
    test_rotations(input, solution, end, SecurityMethod::End)
}

fn test_rotations_any(input: &[&str], solution: usize, end: i64) -> TestResult {
    test_rotations(input, solution, end, SecurityMethod::Any)
}

#[test]
fn test_basic() -> TestResult {
    test_rotations_end(&["R0"], 0, 50)?;
    test_rotations_end(&["L0"], 0, 50)?;
    test_rotations_end(&["R1"], 0, 51)?;
    test_rotations_end(&["L1"], 0, 49)?;
    test_rotations_end(&["R50"], 1, 0)?;
    test_rotations_end(&["L50"], 1, 0)?;
    test_rotations_end(&["R51"], 0, 1)?;
    test_rotations_end(&["L51"], 0, 99)?;

    test_rotations_any(&["R0"], 0, 50)?;
    test_rotations_any(&["L0"], 0, 50)?;
    test_rotations_any(&["R1"], 0, 51)?;
    test_rotations_any(&["L1"], 0, 49)?;
    test_rotations_any(&["R50"], 1, 0)?;
    test_rotations_any(&["L50"], 1, 0)?;
    test_rotations_any(&["R51"], 1, 1)?;
    test_rotations_any(&["L51"], 1, 99)?;
    test_rotations_any(&["R1000"], 10, 50)?;

    Ok(())
}

#[test]
fn test_multiple_rotations() -> TestResult {
    test_rotations_end(&["L51", "L51"], 0, 48)?;
    test_rotations_end(&["L51", "L51", "L49"], 0, 99)?;
    test_rotations_end(&["L50", "L24", "R124"], 2, 0)?;
    test_rotations_end(&["L50", "L24", "L176"], 2, 0)?;

    test_rotations_any(&["L51", "L51"], 1, 48)?;
    test_rotations_any(&["L51", "L51", "L49"], 2, 99)?;
    test_rotations_any(&["L50", "L24"], 1, 76)?;
    test_rotations_any(&["L50", "L24", "R124"], 3, 0)?;
    test_rotations_any(&["L50", "L24", "L176"], 3, 0)?;
    test_rotations_any(&["R26", "L176"], 2, 0)?;
    test_rotations_any(&["R50", "R22"], 1, 22)?;

    Ok(())
}
