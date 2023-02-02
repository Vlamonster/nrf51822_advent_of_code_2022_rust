use crate::data_structures::bitarray::BitArray2D;
use alloc::vec;
use alloc::vec::Vec;
use rtt_target::rprintln;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Measured speed: 165,469us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut rope: [(i32, i32); 2] = [(0, 0); 2];
    let mut direction = Direction::Up;
    let mut steps = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for &byte in input.iter() {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                'outer: for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff == 2 || y_diff == -2 || x_diff == 2 || x_diff == -2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        } else {
                            continue 'outer;
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

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    let mut visited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len());
        BitArray2D::new(pointer, width, height)
    };
    visited.set(0, 0);

    rope.fill((0, 0));
    let mut direction = Direction::Up;
    let mut steps = 0;

    for &byte in input.iter() {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                'outer: for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff == 2 || y_diff == -2 || x_diff == 2 || x_diff == -2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        } else {
                            continue 'outer;
                        }
                    }

                    let &(tx, ty) = rope.last().unwrap();
                    let (rx, ry) = ((tx - min_x) as usize, (ty - min_y) as usize);

                    visited.set(rx, ry);
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }
    rprintln!("d09a: {}", visited.count_ones());
}

/// Measured speed: 328,653us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];
    let mut direction = Direction::Up;
    let mut steps = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for &byte in input.iter() {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                'outer: for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff == 2 || y_diff == -2 || x_diff == 2 || x_diff == -2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        } else {
                            continue 'outer;
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

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    let mut visited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len());
        BitArray2D::new(pointer, width, height)
    };
    visited.set(0, 0);

    rope.fill((0, 0));
    let mut direction = Direction::Up;
    let mut steps = 0;

    for &byte in input.iter() {
        match byte {
            b'U' => direction = Direction::Up,
            b'D' => direction = Direction::Down,
            b'L' => direction = Direction::Left,
            b'R' => direction = Direction::Right,
            b'\n' => {
                'outer: for _ in 0..steps {
                    match direction {
                        Direction::Up => rope[0].1 += 1,
                        Direction::Down => rope[0].1 -= 1,
                        Direction::Left => rope[0].0 -= 1,
                        Direction::Right => rope[0].0 += 1,
                    }
                    for knot in 1..rope.len() {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff == 2 || y_diff == -2 || x_diff == 2 || x_diff == -2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        } else {
                            continue 'outer;
                        }
                    }

                    let &(tx, ty) = rope.last().unwrap();
                    let (rx, ry) = ((tx - min_x) as usize, (ty - min_y) as usize);

                    visited.set(rx, ry);
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }

    rprintln!("d09b: {}", visited.count_ones());
}
