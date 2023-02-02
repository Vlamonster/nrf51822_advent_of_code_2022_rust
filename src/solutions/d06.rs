use rtt_target::rprintln;

/// Measured speed: 5,816us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut hash = 0u32;
    let mut counter = 0;
    let mut window = [0; 4];
    for &byte in input.iter() {
        if counter >= 4 {
            hash ^= 1 << (window[counter & 0b11] - b'a');
        }
        window[counter & 0b11] = byte;
        hash ^= 1 << (byte - b'a');
        counter += 1;

        if hash.count_ones() == 4 {
            break;
        }
    }
    rprintln!("d06a: {}", counter);
}

/// Measured speed: 11,392us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut hash = 0u32;
    let mut counter = 0;
    let mut window_index = 0;
    let mut window = [0; 14];
    for &byte in input.iter() {
        if counter >= 14 {
            if window_index == 14 {
                window_index = 0;
            }
            hash ^= 1 << (window[window_index] - b'a');
        }
        window[window_index] = byte;
        window_index += 1;
        hash ^= 1 << (byte - b'a');
        counter += 1;
        if hash.count_ones() == 14 {
            break;
        }
    }
    rprintln!("d06b: {}", counter);
}
