use alloc::string::String;
use alloc::vec::Vec;
use rtt_target::rprintln;

/// Measured speed: 844us.
pub fn p1(memory: &mut [u8], input: &[u8]) {
    let mut trace = unsafe {
        Vec::from_raw_parts(
            memory
                .as_mut_ptr()
                .add(input.len() + 4 - input.len() % 4)
                .cast::<i32>(),
            0,
            512,
        )
    };

    trace.push(0);

    let mut i = 0;
    let mut sign = 1;
    let mut operand = 0;
    let mut x = 1;
    while i < input.len() {
        match input[i] {
            b'n' => {
                i += 5;
                trace.push_within_capacity(x).unwrap();
            }
            b'a' => i += 5,
            b'-' => {
                sign = -1;
                i += 1;
            }
            b'\n' => {
                trace.push_within_capacity(x).unwrap();
                trace.push_within_capacity(x).unwrap();

                x += sign * operand;

                i += 1;
                sign = 1;
                operand = 0;
            }
            _ => {
                operand = operand * 10 + (input[i] - b'0') as i32;
                i += 1;
            }
        }
    }

    rprintln!(
        "d10a: {}",
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|&cycle| trace[cycle] * cycle as i32)
            .sum::<i32>()
    );

    trace.into_raw_parts();
}

/// Measured speed: 1,820us.
pub fn p2(memory: &mut [u8], input: &[u8]) {
    let mut trace = unsafe {
        Vec::from_raw_parts(
            memory
                .as_mut_ptr()
                .add(input.len() + 4 - input.len() % 4)
                .cast::<i32>(),
            0,
            512,
        )
    };

    trace.push(0);

    let mut i = 0;
    let mut sign = 1;
    let mut operand = 0;
    let mut x = 1;
    while i < input.len() {
        match input[i] {
            b'n' => {
                i += 5;
                trace.push_within_capacity(x).unwrap();
            }
            b'a' => i += 5,
            b'-' => {
                sign = -1;
                i += 1;
            }
            b'\n' => {
                trace.push_within_capacity(x).unwrap();
                trace.push_within_capacity(x).unwrap();

                x += sign * operand;

                i += 1;
                sign = 1;
                operand = 0;
            }
            _ => {
                operand = operand * 10 + (input[i] - b'0') as i32;
                i += 1;
            }
        }
    }

    let mut crt = String::new();

    for y in 0..6 {
        crt.push('\n');
        for x in 1..=40 {
            if (x - 2..=x).contains(&trace[y * 40 + x as usize]) {
                crt.push('#');
            } else {
                crt.push('.');
            }
        }
    }

    rprintln!("d10b: {}", crt);

    trace.into_raw_parts();
}
