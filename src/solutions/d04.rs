use alloc::vec::Vec;
use rtt_target::rprintln;

/// Measured speed: 20110us.
pub fn p1(input: Vec<u8>) {
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    let mut counter = 0;
    let mut total = 0;
    for byte in input {
        match byte {
            b'-' | b',' => counter += 1,
            b'\n' => {
                if (a >= c && b <= d) || (a <= c && b >= d) {
                    total += 1;
                }
                a = 0;
                b = 0;
                c = 0;
                d = 0;
                counter = 0;
            }
            _ => match counter {
                0 => a = a * 10 + (byte - b'0'),
                1 => b = b * 10 + (byte - b'0'),
                2 => c = c * 10 + (byte - b'0'),
                _ => d = d * 10 + (byte - b'0'),
            },
        }
    }
    rprintln!("d04a: {}", total);
}

/// Measured speed: 20433us
pub fn p2(input: Vec<u8>) {
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    let mut counter = 0;
    let mut total = 0;
    for byte in input {
        match byte {
            b'-' | b',' => counter += 1,
            b'\n' => {
                if a <= d && c <= b {
                    total += 1;
                }
                a = 0;
                b = 0;
                c = 0;
                d = 0;
                counter = 0;
            }
            _ => match counter {
                0 => a = a * 10 + (byte - b'0'),
                1 => b = b * 10 + (byte - b'0'),
                2 => c = c * 10 + (byte - b'0'),
                _ => d = d * 10 + (byte - b'0'),
            },
        }
    }
    rprintln!("d04b: {}", total);
}
