use alloc::vec;
use alloc::vec::Vec;
use rtt_target::rprintln;

/// Speed measured: 17689us.
pub fn p1(input: Vec<u8>) {
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
    rprintln!(
        "d07a: {}",
        directories
            .iter()
            .filter(|&&size| size < 100000)
            .sum::<u32>()
    );
}

/// Measured speed: 17873us.
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
    let needed_space = directories[0] - 40000000;
    rprintln!(
        "d07b: {}",
        directories
            .iter()
            .filter(|&&size| size >= needed_space)
            .min()
            .unwrap()
    );
}
