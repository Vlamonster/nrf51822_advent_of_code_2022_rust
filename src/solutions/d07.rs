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

pub fn p2(input: Vec<u8>) {
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
        "d07b: {}",
        directories
            .values()
            .filter(|&&size| size >= directories.get(&vec!["/"]).unwrap() - 40000000)
            .min()
            .unwrap()
    )
}
