use rtt_target::rprintln;

/// Measured speed: 22,228us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut total = 0;
    for backpack in input.split(|&byte| byte == b'\n') {
        // Create variables to store the available lowercase and uppercase letters in each half
        let mut first_half_lowercase = 0u32;
        let mut first_half_uppercase = 0u32;
        let mut second_half_lowercase = 0u32;
        let mut second_half_uppercase = 0u32;

        // Iterate through each character in the first half and mark it as available
        backpack[0..backpack.len() / 2].iter().for_each(|&item| {
            if item >= b'a' {
                first_half_lowercase |= 2 << (item - b'a');
            } else {
                first_half_uppercase |= 2 << (item - b'A');
            }
        });

        // Iterate through each character in the second half and mark it as available
        backpack[backpack.len() / 2..backpack.len()]
            .iter()
            .for_each(|&item| {
                if item >= b'a' {
                    second_half_lowercase |= 2 << (item - b'a');
                } else {
                    second_half_uppercase |= 2 << (item - b'A');
                }
            });

        if second_half_lowercase & first_half_lowercase != 0 {
            total += (second_half_lowercase & first_half_lowercase).trailing_zeros();
        } else if second_half_uppercase & first_half_uppercase != 0 {
            total += 26 + (second_half_uppercase & first_half_uppercase).trailing_zeros();
        }
    }
    rprintln!("d03a: {}", total);
}

/// Measured speed: 21,721us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut total = 0;
    for [elf1, elf2, elf3] in input.split(|&byte| byte == b'\n').array_chunks() {
        let mut elf1_lowercase = 0u32;
        let mut elf1_uppercase = 0u32;
        let mut elf2_lowercase = 0u32;
        let mut elf2_uppercase = 0u32;
        let mut elf3_lowercase = 0u32;
        let mut elf3_uppercase = 0u32;

        elf1.iter().for_each(|&item| {
            if item >= b'a' {
                elf1_lowercase |= 2 << (item - b'a');
            } else {
                elf1_uppercase |= 2 << (item - b'A');
            }
        });

        elf2.iter().for_each(|&item| {
            if item >= b'a' {
                elf2_lowercase |= 2 << (item - b'a');
            } else {
                elf2_uppercase |= 2 << (item - b'A');
            }
        });

        elf3.iter().for_each(|&item| {
            if item >= b'a' {
                elf3_lowercase |= 2 << (item - b'a');
            } else {
                elf3_uppercase |= 2 << (item - b'A');
            }
        });

        if elf1_lowercase & elf2_lowercase & elf3_lowercase != 0 {
            total += (elf1_lowercase & elf2_lowercase & elf3_lowercase).trailing_zeros();
        } else if elf1_uppercase & elf2_uppercase & elf3_uppercase != 0 {
            total += 26 + (elf1_uppercase & elf2_uppercase & elf3_uppercase).trailing_zeros();
        }
    }
    rprintln!("d03b: {}", total);
}
