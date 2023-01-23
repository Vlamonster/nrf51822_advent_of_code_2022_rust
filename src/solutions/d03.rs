use alloc::string::String;
use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

/// Measured speed: 43428us.
pub fn p1(input: Vec<u8>) {
    let mut sum = 0;
    for rucksack in input.split(|&byte| byte == b'\n') {
        let first_half = &rucksack[0..rucksack.len() / 2];
        let second_half = &rucksack[rucksack.len() / 2..rucksack.len()];
        for item in first_half {
            if second_half.contains(item) {
                // Check if the byte is a lowercase or uppercase letter
                if *item >= b'a' {
                    sum += (item - (b'a' - 1)) as u32;
                } else {
                    sum += (item - (b'A' - 27)) as u32;
                }
                // Break out of the loop, as we only need the first matching byte
                break;
            }
        }
    }
    rprintln!("d03a: {}", sum);
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
