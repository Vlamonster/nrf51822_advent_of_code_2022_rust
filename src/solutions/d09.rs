use alloc::vec;
use alloc::vec::Vec;
use core::slice;
use rtt_target::rprintln;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Measured speed: 225,121us.
pub fn p1(memory: &mut [u8], input: &[u8]) {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 2];
    let mut direction = Direction::Up;
    let mut steps = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for &byte in input {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        }
                    }
                    let &(tx, ty) = rope.last().unwrap();
                    min_x = min_x.min(tx);
                    max_x = max_x.max(tx);
                    min_y = min_y.min(ty);
                    max_y = max_y.max(ty);
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }

    let visited = unsafe {
        slice::from_raw_parts_mut(
            memory.as_mut_ptr().add(input.len()),
            memory.len() - input.len(),
        )
    };

    visited.fill(0);

    let width = (max_x - min_x) as usize;

    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 2];
    let mut direction = Direction::Up;
    let mut steps = 0;

    for &byte in input {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        }
                    }

                    let &(tx, ty) = rope.last().unwrap();

                    let rx = (tx - min_x) as usize;
                    let ry = (ty - min_y) as usize;

                    let bit_index = ry * width + rx;
                    let byte_index = bit_index / 8;
                    let inner_index = bit_index % 8; // maybe and?

                    visited[byte_index] |= 1 << inner_index;
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }

    rprintln!(
        "d09a: {}",
        visited.iter().map(|byte| byte.count_ones()).sum::<u32>()
    );
}

pub fn p2(memory: &mut [u8], input: &[u8]) {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 10];
    let mut direction = Direction::Up;
    let mut steps = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for &byte in input {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        }
                    }
                    let &(tx, ty) = rope.last().unwrap();
                    min_x = min_x.min(tx);
                    max_x = max_x.max(tx);
                    min_y = min_y.min(ty);
                    max_y = max_y.max(ty);
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }

    let visited = unsafe {
        slice::from_raw_parts_mut(
            memory.as_mut_ptr().add(input.len()),
            memory.len() - input.len(),
        )
    };

    visited.fill(0);

    let width = (max_x - min_x) as usize;

    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 10];
    let mut direction = Direction::Up;
    let mut steps = 0;

    for &byte in input {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        }
                    }

                    let &(tx, ty) = rope.last().unwrap();

                    let rx = (tx - min_x) as usize;
                    let ry = (ty - min_y) as usize;

                    let bit_index = ry * width + rx;
                    let byte_index = bit_index / 8;
                    let inner_index = bit_index % 8; // maybe and?

                    visited[byte_index] |= 1 << inner_index;
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }

    rprintln!(
        "d09b: {}",
        visited.iter().map(|byte| byte.count_ones()).sum::<u32>()
    );
}
