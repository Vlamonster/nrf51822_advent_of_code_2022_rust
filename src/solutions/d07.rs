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
    let mut directories = BTreeMap::<Vec<_>, u32>::new();
    let root: Vec<&[u8]> = vec![b"/"];
    let mut path = Vec::new();
    for line in input.split(|&d| d == b'\n') {
        match line {
            b"$ cd /" => {
                path = root.clone();
                directories.insert(path.clone(), 0);
            }
            b"$ cd .." => {
                path.pop();
            }
            _ if line.starts_with(b"$ cd ") => {
                let directory = &line[5..];
                path.push(directory);
                directories.insert(path.clone(), 0);
            }
            b"$ ls" | b"" => {}
            _ if line.starts_with(b"dir") => {}
            _ => {
                let file_size = line
                    .split(|&d| d == b' ')
                    .next()
                    .unwrap()
                    .iter()
                    .map(|&d| d as char)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                for i in 1..=path.len() {
                    directories.get_mut(&path[..i]).map(|v| *v += file_size);
                }
            }
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
    )
}
