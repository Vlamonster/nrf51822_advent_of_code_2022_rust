use alloc::collections::VecDeque;
use core::slice;
use rtt_target::rprintln;

/// Measured speed: 200,008us.
pub fn p1(memory: &mut [u8], input: &mut [u8]) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let visited = unsafe {
        slice::from_raw_parts_mut(
            memory.as_mut_ptr().add(input.len()),
            (width * height - 1) / 8 + 1,
        )
    };

    let mut unvisited = VecDeque::new();

    let (mut sx, mut sy) = (0, 0);
    'outer: for y in 0..height {
        for x in 0..width {
            if input[y * (width + 1) + x] == b'S' {
                (sx, sy) = (x, y);
                input[y * (width + 1) + x] = b'a';
                break 'outer;
            }
        }
    }

    let (mut tx, mut ty) = (0, 0);
    'outer: for y in 0..height {
        for x in 0..width {
            if input[y * (width + 1) + x] == b'E' {
                (tx, ty) = (x, y);
                input[y * (width + 1) + x] = b'z';
                break 'outer;
            }
        }
    }

    unvisited.push_back((sx as u8, sy as u8, 0));
    let total_steps;

    loop {
        let (x, y, steps) = unvisited.pop_front().unwrap();
        let (x, y) = (x as usize, y as usize);
        let byte = input[y * (width + 1) + x];

        if (x, y) == (tx, ty) {
            total_steps = steps;
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if !(0..width).contains(&nx) || !(0..height).contains(&ny) {
                continue;
            }

            if input[ny * (width + 1) + nx] > byte + 1 {
                continue;
            }

            let bit_index = ny * width + nx;
            let byte_index = bit_index / 8;
            let inner_index = (bit_index % 8) as u8;

            if visited[byte_index] & 1 << inner_index != 0 {
                continue;
            } else {
                visited[byte_index] |= 1 << inner_index;
            }

            unvisited.push_back((nx as u8, ny as u8, steps + 1));
        }
    }

    rprintln!("d12a: {}", total_steps);
}

pub fn p2(_memory: &mut [u8], _input: &mut [u8]) {
    rprintln!("d12b: {}", "nyi");
}
