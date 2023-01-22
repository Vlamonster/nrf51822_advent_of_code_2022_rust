use alloc::string::String;
use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;
use itertools::Itertools;

pub fn p1(input: Vec<u8>) {
    hprintln!(
        "d03a: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .map(|rucksack| {
                let (first, second) = rucksack.split_at(rucksack.len() / 2);
                first.chars().find(|&item| second.contains(item)).unwrap()
            })
            .map(|misplaced_item| match misplaced_item {
                'a'..='z' => misplaced_item as usize - b'a' as usize + 1,
                'A'..='Z' => misplaced_item as usize - b'A' as usize + 27,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}

pub fn p2(input: Vec<u8>) {
    hprintln!(
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
