use alloc::string::String;
use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;
use itertools::Itertools;

pub fn p1(input: Vec<u8>) {
    hprintln!(
        "d01a: {}",
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
            .max()
            .unwrap()
    );
}

pub fn p2(input: Vec<u8>) {
    hprintln!(
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
