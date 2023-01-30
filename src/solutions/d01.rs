use rtt_target::rprintln;

/// Measured speed: 11695us.
pub fn p1(_memory: &mut [u8], input: &[u8]) {
    // Initialize variables to keep track of maximum sum, current group sum and current line sum
    let mut max_sum = 0;
    let mut current_group_sum = 0;
    let mut current_line_sum = 0;

    // Initialize variable to keep track of whether the last byte encountered was a newline
    let mut last_byte_was_newline = false;

    for &byte in input {
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

/// Measured speed: 11964us.
pub fn p2(_memory: &mut [u8], input: &[u8]) {
    // Initialize variables to keep track of highest sums, current group sum and current line sum
    let mut highest_sums = [0, 0, 0];
    let mut current_group_sum = 0;
    let mut current_line_sum = 0;

    // Initialize variable to keep track of whether the last byte encountered was a newline
    let mut last_byte_was_newline = false;

    for &byte in input {
        match (byte, last_byte_was_newline) {
            // If the current byte is a newline and the last byte was also a newline
            (b'\n', true) => {
                highest_sums.sort();
                highest_sums[0] = highest_sums[0].max(current_group_sum);
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
    rprintln!(
        "d01b: {}",
        highest_sums[0] + highest_sums[1] + highest_sums[2]
    );
}
