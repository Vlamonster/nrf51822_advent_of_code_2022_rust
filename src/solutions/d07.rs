use alloc::vec::Vec;
use rtt_target::rprintln;

/// Speed measured: 17689us.
pub fn p1(memory: &mut [u8], input: &[u8]) {
    let mut directories = unsafe {
        Vec::from_raw_parts(
            memory
                .as_mut_ptr()
                .add(input.len() + 4 - input.len() % 4)
                .cast::<u32>(),
            0,
            1000,
        )
    };
    let mut path = unsafe {
        Vec::from_raw_parts(
            directories
                .as_mut_ptr()
                .add(directories.capacity())
                .cast::<usize>(),
            0,
            1000,
        )
    };
    directories.push(0);
    path.push(0);
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

    directories.into_raw_parts();
    path.into_raw_parts();
}

/// Measured speed: 17873us.
pub fn p2(memory: &mut [u8], input: &[u8]) {
    let mut directories = unsafe {
        Vec::from_raw_parts(
            memory
                .as_mut_ptr()
                .add(input.len() + 4 - input.len() % 4)
                .cast::<u32>(),
            0,
            1000,
        )
    };
    let mut path = unsafe {
        Vec::from_raw_parts(
            directories
                .as_mut_ptr()
                .add(directories.capacity())
                .cast::<usize>(),
            0,
            1000,
        )
    };
    directories.push(0);
    path.push(0);
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

    directories.into_raw_parts();
    path.into_raw_parts();
}
