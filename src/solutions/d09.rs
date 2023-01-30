use alloc::vec;
use alloc::vec::Vec;
use rtt_target::rprintln;

const TL: u8 = 0b10000000;
const TM: u8 = 0b01000000;
const TR: u8 = 0b00100000;
const ML: u8 = 0b00010000;
const MR: u8 = 0b00001000;
const BL: u8 = 0b00000100;
const BM: u8 = 0b00000010;
const BR: u8 = 0b00000001;

/// Measured speed: 46,193,655us.
pub fn p1(memory: &mut [u8], input: &[u8]) {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 2];
    let mut tail_motions =
        unsafe { Vec::from_raw_parts(memory.as_mut_ptr().add(input.len()), 0, 10000) };
    let mut direction = 0;
    let mut steps = 0;

    for byte in input.into_iter() {
        match byte {
            b'U' => direction = TM,
            b'D' => direction = BM,
            b'L' => direction = ML,
            b'R' => direction = MR,
            b'\n' => {
                for _ in 0..steps {
                    match direction {
                        TM => rope[0].1 += 1,
                        BM => rope[0].1 -= 1,
                        ML => rope[0].0 -= 1,
                        MR => rope[0].0 += 1,
                        _ => {}
                    }
                    for knot in 1..rope.len() - 1 {
                        let x_diff = rope[knot - 1].0 - rope[knot].0;
                        let y_diff = rope[knot - 1].1 - rope[knot].1;

                        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                            rope[knot].0 += x_diff.signum();
                            rope[knot].1 += y_diff.signum();
                        }
                    }
                    let tail = rope.len() - 1;
                    let x_diff = rope[tail - 1].0 - rope[tail].0;
                    let y_diff = rope[tail - 1].1 - rope[tail].1;

                    if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
                        rope[tail].0 += x_diff.signum();
                        rope[tail].1 += y_diff.signum();
                        match (x_diff.signum(), y_diff.signum()) {
                            (-1, 1) => tail_motions.push_within_capacity(TL),
                            (0, 1) => tail_motions.push_within_capacity(TM),
                            (1, 1) => tail_motions.push_within_capacity(TR),
                            (-1, 0) => tail_motions.push_within_capacity(ML),
                            (1, 0) => tail_motions.push_within_capacity(MR),
                            (-1, -1) => tail_motions.push_within_capacity(BL),
                            (0, -1) => tail_motions.push_within_capacity(BM),
                            (_, _) => tail_motions.push_within_capacity(BR),
                        }
                        .unwrap();
                    }
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }
    let mut new_motions = unsafe { Vec::from_raw_parts(memory.as_mut_ptr(), 0, 10000) };
    for &motion in &tail_motions {
        new_motions.push_within_capacity(motion).unwrap();
    }
    tail_motions.into_raw_parts();
    let mut tail_motions = new_motions;
    let spare_space = (memory.len() - tail_motions.capacity()) / 4;
    let mut tail_positions = unsafe {
        Vec::from_raw_parts(
            tail_motions
                .as_mut_ptr()
                .add(tail_motions.capacity() + 4 - tail_motions.capacity() % 4)
                .cast::<(i16, i16)>(),
            0,
            spare_space,
        )
    };

    let mut visited = 1;
    let (mut x, mut y) = (0, 0);
    for i in 0..tail_motions.len() {
        match tail_motions[i] {
            TL => (x, y) = (x - 1, y + 1),
            TM => (x, y) = (x, y + 1),
            TR => (x, y) = (x + 1, y + 1),
            ML => (x, y) = (x - 1, y),
            MR => (x, y) = (x + 1, y),
            BL => (x, y) = (x - 1, y - 1),
            BM => (x, y) = (x, y - 1),
            _ => (x, y) = (x + 1, y - 1),
        }
        if i < spare_space {
            if !tail_positions.contains(&(x, y)) {
                visited += 1;
            }
            tail_positions.push_within_capacity((x, y)).unwrap();
        } else {
            let (mut nx, mut ny) = tail_positions.last().unwrap();
            if !tail_positions.contains(&(x, y))
                && (spare_space..i).all(|j| {
                    match tail_motions[j] {
                        TL => (nx, ny) = (nx - 1, ny + 1),
                        TM => (nx, ny) = (nx, ny + 1),
                        TR => (nx, ny) = (nx + 1, ny + 1),
                        ML => (nx, ny) = (nx - 1, ny),
                        MR => (nx, ny) = (nx + 1, ny),
                        BL => (nx, ny) = (nx - 1, ny - 1),
                        BM => (nx, ny) = (nx, ny - 1),
                        _ => (nx, ny) = (nx + 1, ny - 1),
                    };
                    (x, y) != (nx, ny)
                })
            {
                visited += 1;
            }
        }
    }
    rprintln!("d09a: {}", visited);

    tail_motions.into_raw_parts();
    tail_positions.into_raw_parts();
}

pub fn p2(_memory: &mut [u8], _input: &[u8]) {
    rprintln!("d09b: {}", "nyi");
}
