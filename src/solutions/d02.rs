use itertools::Itertools;
use rtt_target::rprintln;

/// Measured speed: 5,501us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut total_score = 0u32;
    for (opponent, byte) in input.iter().step_by(2).tuples() {
        match (opponent, byte) {
            (b'A', b'X') => total_score += 1 + 3,
            (b'A', b'Y') => total_score += 2 + 6,
            (b'A', b'Z') => total_score += 3,
            (b'B', b'X') => total_score += 1,
            (b'B', b'Y') => total_score += 2 + 3,
            (b'B', b'Z') => total_score += 3 + 6,
            (b'C', b'X') => total_score += 1 + 6,
            (b'C', b'Y') => total_score += 2,
            (b'C', b'Z') => total_score += 3 + 3,
            _ => {}
        }
    }
    rprintln!("d02a: {}", total_score);
}

/// Measured speed: 5,501us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut total_score = 0u32;
    for (opponent, byte) in input.iter().step_by(2).tuples() {
        match (opponent, byte) {
            (b'A', b'X') => total_score += 3,
            (b'A', b'Y') => total_score += 1 + 3,
            (b'A', b'Z') => total_score += 2 + 6,
            (b'B', b'X') => total_score += 1,
            (b'B', b'Y') => total_score += 2 + 3,
            (b'B', b'Z') => total_score += 3 + 6,
            (b'C', b'X') => total_score += 2,
            (b'C', b'Y') => total_score += 3 + 3,
            (b'C', b'Z') => total_score += 1 + 6,
            _ => {}
        }
    }
    rprintln!("d02b: {}", total_score);
}
