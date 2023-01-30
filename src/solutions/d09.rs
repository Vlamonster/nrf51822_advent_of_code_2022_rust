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

/// Measured speed: 82,059,566us.
pub fn p1(input: Vec<u8>) {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); 2];
    let mut tail_motions = Vec::with_capacity(12000);
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
                            (-1, 1) => tail_motions.push(TL),
                            (0, 1) => tail_motions.push(TM),
                            (1, 1) => tail_motions.push(TR),
                            (-1, 0) => tail_motions.push(ML),
                            (1, 0) => tail_motions.push(MR),
                            (-1, -1) => tail_motions.push(BL),
                            (0, -1) => tail_motions.push(BM),
                            (_, _) => tail_motions.push(BR),
                        }
                    }
                }
                steps = 0;
            }
            b' ' => {}
            _ => steps = steps * 10 + (byte - b'0'),
        }
    }
    let mut visited = 1;
    let (mut x, mut y) = (0, 0);
    'outer: for i in 0..tail_motions.len() {
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
        let (mut nx, mut ny) = (0, 0);
        for j in 0..i {
            match tail_motions[j] {
                TL => (nx, ny) = (nx - 1, ny + 1),
                TM => (nx, ny) = (nx, ny + 1),
                TR => (nx, ny) = (nx + 1, ny + 1),
                ML => (nx, ny) = (nx - 1, ny),
                MR => (nx, ny) = (nx + 1, ny),
                BL => (nx, ny) = (nx - 1, ny - 1),
                BM => (nx, ny) = (nx, ny - 1),
                _ => (nx, ny) = (nx + 1, ny - 1),
            }
            if (x, y) == (nx, ny) {
                continue 'outer;
            }
        }
        visited += 1;
    }
    rprintln!("d09a: {}", visited);
}

pub fn p2(_memory: &mut [u8], _input: &[u8]) {
    rprintln!("d09b: {}", "nyi");
}
