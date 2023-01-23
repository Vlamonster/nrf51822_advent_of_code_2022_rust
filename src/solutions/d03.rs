use alloc::string::String;
use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

/// Measured speed: 22364us.
pub fn p1(input: Vec<u8>) {
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

pub fn p2(input: Vec<u8>) {
    rprintln!(
        "d03b: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .tuples::<(_, _, _)>()
            .map(|(elf_1, elf_2, elf_3)| {
                elf_1
                    .chars()
                    .find(|&item| elf_2.contains(item) && elf_3.contains(item))
                    .unwrap()
            })
            .map(|badge| match badge {
                'a'..='z' => badge as usize - b'a' as usize + 1,
                'A'..='Z' => badge as usize - b'A' as usize + 27,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}
