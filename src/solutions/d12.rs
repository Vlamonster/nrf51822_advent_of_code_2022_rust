use crate::data_structures::bitarray::BitArray2D;
use crate::data_structures::ringbuffer::RingBuffer;
use rtt_target::rprintln;

/// Measured speed: 200,008us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut visited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len());
        BitArray2D::new(pointer, width, height)
    };
    let mut unvisited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len() + visited.len());
        let offset = pointer.align_offset(2);
        RingBuffer::new(pointer.add(offset).cast::<(u8, u8, u16)>(), 1 << 10)
    };

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

    unvisited.queue((sx as u8, sy as u8, 0));
    let total_steps;

    loop {
        let (x, y, steps) = unvisited.dequeue();
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

            if visited.get(nx, ny) {
                continue;
            } else {
                visited.set(nx, ny)
            }

            unvisited.queue((nx as u8, ny as u8, steps + 1));
        }
    }

    rprintln!("d12a: {}", total_steps);
}

/// Measured speed: 259,865us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let width = input.iter().position(|&d| d == b'\n').unwrap();
    let height = input.iter().rposition(|&d| d == b'\n').unwrap() / width;

    let mut visited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len());
        BitArray2D::new(pointer, width, height)
    };
    let mut unvisited = unsafe {
        let pointer = input.as_mut_ptr().add(input.len() + visited.len());
        let offset = pointer.align_offset(2);
        RingBuffer::new(pointer.add(offset).cast::<(u8, u8, u16)>(), 1 << 12)
    };

    'outer: for y in 0..height {
        for x in 0..width {
            if input[y * (width + 1) + x] == b'S' {
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

    for y in 0..height {
        for x in 0..width {
            if input[y * (width + 1) + x] == b'a' {
                unvisited.queue((x as u8, y as u8, 0));
            }
        }
    }

    let total_steps;
    loop {
        let (x, y, steps) = unvisited.dequeue();
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

            if visited.get(nx, ny) {
                continue;
            } else {
                visited.set(nx, ny)
            }

            unvisited.queue((nx as u8, ny as u8, steps + 1));
        }
    }

    rprintln!("d12b: {}", total_steps);
}
