use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

pub fn p1(input: Vec<u8>) {
    let contents = input.into_iter().map(|d| d as char).collect::<String>();
    let mut directories = BTreeMap::<Vec<&str>, u32>::new();
    let mut path = vec![];
    for line in contents.lines() {
        match line.split_whitespace().collect_vec().as_slice() {
            ["$", "cd", "/"] => {
                path = vec!["/"];
                directories.insert(path.clone(), 0);
            }
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", directory] => {
                path.push(directory);
                directories.insert(path.clone(), 0);
            }
            ["dir", _] | ["$", "ls"] => {}
            [file_size, _] => {
                let path_clone = path.clone();
                for _ in 0..path.len() {
                    *directories.get_mut(&path).unwrap() += file_size.parse::<u32>().unwrap();
                    path.pop();
                }
                path = path_clone;
            }
            _ => unreachable!(),
        }
    }
    rprintln!(
        "d07a: {}",
        directories
            .values()
            .filter(|&&size| size < 100000)
            .sum::<u32>()
    )
}

/// Measured speed: 28722us.
pub fn p2(input: Vec<u8>) {
    let mut directories = vec![0];
    let mut path = vec![0];
    for line in input.split(|&d| d == b'\n') {
        match line {
            b"$ cd .." => {
                let inner = directories[path.pop().unwrap()];
                directories[*path.last().unwrap()] += inner;
            }
            b"$ cd /" | b"" | b"$ ls" => {}
            _ if line.starts_with(b"dir") => {}
            _ if line.starts_with(b"$ cd ") => {
                path.push(directories.len());
                directories.push(0);
            }
            _ => {
                let mut file_size = 0;
                for &byte in line {
                    if byte == b' ' {
                        break;
                    } else {
                        file_size = file_size * 10 + (byte - b'0') as u32;
                    }
                }
                directories[*path.last().unwrap()] += file_size;
            }
        }
    }
    while let Some(index) = path.pop() {
        if let Some(&last) = path.last() {
            directories[last] += directories[index];
        }
    }
    let needed_space = directories.get(&root).unwrap() - 40000000;
    rprintln!(
        "d07b: {}",
        directories
            .values()
            .filter(|&&size| size >= needed_space)
            .min()
            .unwrap()
    );
}
