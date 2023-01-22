use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;

pub fn p1(input: Vec<u8>) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut visible = 0;
    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                visible += 1;
                continue;
            }
            if (0..x).all(|nx| input[y * (width + 1) + nx] < input[y * (width + 1) + x]) {
                visible += 1;
                continue;
            }
            if (x + 1..width).all(|nx| input[y * (width + 1) + nx] < input[y * (width + 1) + x]) {
                visible += 1;
                continue;
            }
            if (0..y).all(|ny| input[ny * (width + 1) + x] < input[y * (width + 1) + x]) {
                visible += 1;
                continue;
            }
            if (y + 1..height).all(|ny| input[ny * (width + 1) + x] < input[y * (width + 1) + x]) {
                visible += 1;
                continue;
            }
        }
    }

    hprintln!("d08a: {}", visible);
}

pub fn p2(input: Vec<u8>) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut best_score = 0;
    for x in 0..width {
        for y in 0..height {
            best_score = best_score.max(
                (0..x)
                    .rev()
                    .position(|nx| input[y * (width + 1) + nx] >= input[y * (width + 1) + x])
                    .unwrap_or_else(|| x.wrapping_sub(1))
                    .wrapping_add(1)
                    * (x + 1..width)
                        .position(|nx| input[y * (width + 1) + nx] >= input[y * (width + 1) + x])
                        .unwrap_or_else(|| (width - x - 1).wrapping_sub(1))
                        .wrapping_add(1)
                    * (0..y)
                        .rev()
                        .position(|ny| input[ny * (width + 1) + x] >= input[y * (width + 1) + x])
                        .unwrap_or_else(|| y.wrapping_sub(1))
                        .wrapping_add(1)
                    * (y + 1..height)
                        .position(|ny| input[ny * (width + 1) + x] >= input[y * (width + 1) + x])
                        .unwrap_or_else(|| (height - y - 1).wrapping_sub(1))
                        .wrapping_add(1),
            );
        }
    }

    hprintln!("d08b: {}", best_score);
}
