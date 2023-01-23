use alloc::vec::Vec;
use rtt_target::rprintln;

/// Measured speed: 14777us.
pub fn p1(input: Vec<u8>) {
    // Initialize variable to keep track of the total score
    let mut total_score = 0u32;

    // Initialize variable to keep track of the last played shape by opponent
    let mut opponent = b'\n';

    for byte in input {
        match (opponent, byte) {
            // If byte is one of [b'A', b'B', b'C'] then this is opponent's shape
            (_, b'A') => opponent = b'A',
            (_, b'B') => opponent = b'B',
            (_, b'C') => opponent = b'C',
            // If byte is one of [b'X', b'Y', b'Z'] then add the score of the game to the total
            (b'A', b'X') => total_score += 1 + 3,
            (b'A', b'Y') => total_score += 2 + 6,
            (b'A', b'Z') => total_score += 3,
            (b'B', b'X') => total_score += 1,
            (b'B', b'Y') => total_score += 2 + 3,
            (b'B', b'Z') => total_score += 3 + 6,
            (b'C', b'X') => total_score += 1 + 6,
            (b'C', b'Y') => total_score += 2,
            (b'C', b'Z') => total_score += 3 + 3,
            // Otherwise byte is not a shape
            _ => {}
        }
    }
    rprintln!("d02a: {}", total_score);
}

/// Measured speed: 14777us.
pub fn p2(input: Vec<u8>) {
    // Initialize variable to keep track of the total score
    let mut total_score = 0u32;

    // Initialize variable to keep track of the last played shape by opponent
    let mut opponent = b'\n';

    for byte in input {
        match (opponent, byte) {
            // If byte is one of [b'A', b'B', b'C'] then this is opponent's shape
            (_, b'A') => opponent = b'A',
            (_, b'B') => opponent = b'B',
            (_, b'C') => opponent = b'C',
            // If byte is one of [b'X', b'Y', b'Z'] then add the score of the game to the total
            (b'A', b'X') => total_score += 3,
            (b'A', b'Y') => total_score += 1 + 3,
            (b'A', b'Z') => total_score += 2 + 6,
            (b'B', b'X') => total_score += 1,
            (b'B', b'Y') => total_score += 2 + 3,
            (b'B', b'Z') => total_score += 3 + 6,
            (b'C', b'X') => total_score += 2,
            (b'C', b'Y') => total_score += 3 + 3,
            (b'C', b'Z') => total_score += 1 + 6,
            // Otherwise byte is not a shape
            _ => {}
        }
    }
    rprintln!("d02b: {}", total_score);
}
