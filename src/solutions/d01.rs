use alloc::string::String;
use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

/// Measured speed: 11190us.
pub fn p1(input: Vec<u8>) {
    // Initialize variables to keep track of maximum sum, current group sum and current line sum
    let mut max_sum = 0;
    let mut current_group_sum = 0;
    let mut current_line_sum = 0;

    // Initialize variable to keep track of whether the last byte encountered was a newline
    let mut last_byte_was_newline = false;

    for byte in input {
        match (byte, last_byte_was_newline) {
            // If the current byte is a newline and the last byte was also a newline
            (b'\n', true) => {
                max_sum = max_sum.max(current_group_sum);
                current_group_sum = 0;
                current_line_sum = 0;
                last_byte_was_newline = false;
            }
            // If the current byte is a newline and the last byte wasn't a newline
            (b'\n', _) => {
                current_group_sum += current_line_sum;
                current_line_sum = 0;
                last_byte_was_newline = true;
            }
            // Otherwise the current byte is a digit
            (_, _) => {
                current_line_sum = current_line_sum * 10 + (byte - b'0') as u32;
                last_byte_was_newline = false;
            }
        }
    }
    rprintln!("d01a: {}", max_sum);
}

pub fn p2(input: Vec<u8>) {
    rprintln!(
        "d01b: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|calories| calories.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
    );
}
