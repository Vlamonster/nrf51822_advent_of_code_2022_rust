use alloc::vec::Vec;
use rtt_target::rprintln;

pub fn p1(input: Vec<u8>) {
    let mut sums = Vec::new();
    let mut newline = false;
    let mut sum = 0;
    let mut sub = 0;
    for d in input.into_iter() {
        match (d, newline) {
            (b'\n', true) => {
                sums.push(sum);
                sum = 0;
                sub = 0;
                newline = false;
            }
            (b'\n', _) => {
                sum += sub;
                sub = 0;
                newline = true;
            }
            (d, _) => {
                sub = sub * 10 + (d - b'0') as u32;
                newline = false;
            }
        }
    }
    rprintln!("{}", sums.into_iter().max().unwrap());
}

pub fn p2(input: &[u8]) {
    rprintln!("not yet implemented p2");
}
