use alloc::string::String;
use alloc::vec::Vec;
use rtt_target::rprintln;

pub fn p1(input: Vec<u8>) {
    rprintln!(
        "d02a: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .map(|game| match game {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2,
                "C Z" => 3 + 3,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}

pub fn p2(input: Vec<u8>) {
    rprintln!(
        "d02b: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .map(|game| match game {
                "A X" => 3,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}
