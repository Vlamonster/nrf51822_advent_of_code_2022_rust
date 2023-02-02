use itertools::Itertools;
use rtt_target::rprintln;

/// Measured speed: 5,501us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut score = 0u32;
    for (opponent, santa) in input.iter().step_by(2).tuples() {
        match (opponent, santa) {
            (b'A', b'X') => score += 1 + 3,
            (b'A', b'Y') => score += 2 + 6,
            (b'A', b'Z') => score += 3,
            (b'B', b'X') => score += 1,
            (b'B', b'Y') => score += 2 + 3,
            (b'B', b'Z') => score += 3 + 6,
            (b'C', b'X') => score += 1 + 6,
            (b'C', b'Y') => score += 2,
            (b'C', b'Z') => score += 3 + 3,
            _ => {}
        }
    }
    rprintln!("d02a: {}", score);
}

/// Measured speed: 5,501us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut score = 0u32;
    for (opponent, santa) in input.iter().step_by(2).tuples() {
        match (opponent, santa) {
            (b'A', b'X') => score += 3,
            (b'A', b'Y') => score += 1 + 3,
            (b'A', b'Z') => score += 2 + 6,
            (b'B', b'X') => score += 1,
            (b'B', b'Y') => score += 2 + 3,
            (b'B', b'Z') => score += 3 + 6,
            (b'C', b'X') => score += 2,
            (b'C', b'Y') => score += 3 + 3,
            (b'C', b'Z') => score += 1 + 6,
            _ => {}
        }
    }
    rprintln!("d02b: {}", score);
}
