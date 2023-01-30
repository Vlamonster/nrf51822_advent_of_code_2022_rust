use rtt_target::rprintln;

/// Measured speed: 29327us.
pub fn p1(_memory: &mut [u8], input: &[u8]) {
    let mut total = 0;
    for backpack in input.split(|&byte| byte == b'\n') {
        // Split the backpack string into the first and second half
        let first_half = &backpack[0..backpack.len() / 2];
        let second_half = &backpack[backpack.len() / 2..backpack.len()];

        // Create variables to store the available lowercase and uppercase letters in each half
        let mut first_half_lowercase = 0u32;
        let mut first_half_uppercase = 0u32;
        let mut second_half_lowercase = 0u32;
        let mut second_half_uppercase = 0u32;

        // Iterate through each character in the first half and mark it as available
        for item in first_half {
            if item.is_ascii_lowercase() {
                first_half_lowercase |= 2 << (item - b'a');
            } else {
                first_half_uppercase |= 2 << (item - b'A');
            }
        }

        // Iterate through each character in the second half and mark it as available
        for item in second_half {
            if item.is_ascii_lowercase() {
                second_half_lowercase |= 2 << (item - b'a');
            } else {
                second_half_uppercase |= 2 << (item - b'A');
            }
        }

        if second_half_lowercase & first_half_lowercase != 0 {
            total += (second_half_lowercase & first_half_lowercase).trailing_zeros();
        } else if second_half_uppercase & first_half_uppercase != 0 {
            total += 26 + (second_half_uppercase & first_half_uppercase).trailing_zeros();
        }
    }
    rprintln!("d03a: {}", total);
}

/// Measured speed: 28986us.
pub fn p2(_memory: &mut [u8], input: &[u8]) {
    let mut total = 0;
    for [elf1, elf2, elf3] in input.split(|&byte| byte == b'\n').array_chunks() {
        let mut elf1_lowercase = 0u32;
        let mut elf1_uppercase = 0u32;
        let mut elf2_lowercase = 0u32;
        let mut elf2_uppercase = 0u32;
        let mut elf3_lowercase = 0u32;
        let mut elf3_uppercase = 0u32;

        for item in elf1 {
            if item.is_ascii_lowercase() {
                elf1_lowercase |= 2 << (item - b'a');
            } else {
                elf1_uppercase |= 2 << (item - b'A');
            }
        }

        for item in elf2 {
            if item.is_ascii_lowercase() {
                elf2_lowercase |= 2 << (item - b'a');
            } else {
                elf2_uppercase |= 2 << (item - b'A');
            }
        }

        for item in elf3 {
            if item.is_ascii_lowercase() {
                elf3_lowercase |= 2 << (item - b'a');
            } else {
                elf3_uppercase |= 2 << (item - b'A');
            }
        }

        if elf1_lowercase & elf2_lowercase & elf3_lowercase != 0 {
            total += (elf1_lowercase & elf2_lowercase & elf3_lowercase).trailing_zeros();
        } else if elf1_uppercase & elf2_uppercase & elf3_uppercase != 0 {
            total += 26 + (elf1_uppercase & elf2_uppercase & elf3_uppercase).trailing_zeros();
        }
    }
    rprintln!("d03b: {}", total);
}
