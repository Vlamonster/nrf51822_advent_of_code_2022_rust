use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;
use itertools::Itertools;

pub fn p1(input: Vec<u8>) {
    let contents = input.into_iter().map(|d| d as char).collect::<String>();
    let (stacks_str, instructions) = contents.split_once("\n\n").unwrap();

    let num_stacks = stacks_str
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];
    for layer in stacks_str.lines().rev().skip(1) {
        for (index, label) in layer.chars().skip(1).step_by(4).enumerate() {
            if label.is_alphabetic() {
                stacks[index].push(label);
            }
        }
    }

    for instruction in instructions.lines() {
        let (moves, from, to) = instruction
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        for _ in 0..moves {
            let moved_crate = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(moved_crate);
        }
    }

    hprintln!(
        "d05a: {}",
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    )
}

pub fn p2(input: Vec<u8>) {
    let contents = input.into_iter().map(|d| d as char).collect::<String>();
    let (stacks_str, instructions) = contents.split_once("\n\n").unwrap();

    let num_stacks = stacks_str
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];
    for layer in stacks_str.lines().rev().skip(1) {
        for (index, label) in layer.chars().skip(1).step_by(4).enumerate() {
            if label.is_alphabetic() {
                stacks[index].push(label);
            }
        }
    }

    let mut moved_crates = Vec::new();
    for instruction in instructions.lines() {
        let (moves, from, to) = instruction
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        for _ in 0..moves {
            moved_crates.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..moves {
            stacks[to - 1].push(moved_crates.pop().unwrap());
        }
    }

    hprintln!(
        "d05b: {}",
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    )
}
