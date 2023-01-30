use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use rtt_target::rprintln;

const VD: VecDeque<u8> = VecDeque::new();

/// Measured speed: 29233us.
pub fn p1(_memory: &mut [u8], input: &[u8]) {
    let mut reading_stacks = true;
    let mut stacks = [VD; 9];
    for line in input.split(|&byte| byte == b'\n') {
        if reading_stacks {
            let mut index = 0;
            for &byte in line {
                match byte {
                    b'9' => reading_stacks = false,
                    _ => {
                        if byte.is_ascii_uppercase() {
                            stacks[index / 4].push_front(byte);
                        }
                        index += 1;
                    }
                }
            }
        } else {
            let mut in_num = false;
            let mut moves = 0;
            let mut from = 0;
            let mut to = 0;
            for &byte in line {
                match byte {
                    b'0'..=b'9' => {
                        if moves == 0 || in_num {
                            moves = moves * 10 + (byte - b'0') as usize;
                            in_num = true;
                        } else if from == 0 {
                            from = (byte - b'0') as usize;
                        } else {
                            to = (byte - b'0') as usize;
                        }
                    }
                    _ => in_num = false,
                }
            }
            for _ in 0..moves {
                let moved_crate = stacks[from - 1].pop_back().unwrap();
                stacks[to - 1].push_back(moved_crate);
            }
        }
    }
    rprintln!(
        "d05a: {}",
        stacks
            .into_iter()
            .map(|stack| *stack.back().unwrap() as char)
            .collect::<String>()
    );
}

/// Measured speed: 33276us.
pub fn p2(_memory: &mut [u8], input: &[u8]) {
    let mut reading_stacks = true;
    let mut stacks = [VD; 9];
    let mut moved_crates = Vec::new();
    for line in input.split(|&byte| byte == b'\n') {
        if reading_stacks {
            let mut index = 0;
            for &byte in line {
                match byte {
                    b'9' => reading_stacks = false,
                    _ => {
                        if byte.is_ascii_uppercase() {
                            stacks[index / 4].push_front(byte);
                        }
                        index += 1;
                    }
                }
            }
        } else {
            let mut in_num = false;
            let mut moves = 0;
            let mut from = 0;
            let mut to = 0;
            for &byte in line {
                match byte {
                    b'0'..=b'9' => {
                        if moves == 0 || in_num {
                            moves = moves * 10 + (byte - b'0') as usize;
                            in_num = true;
                        } else if from == 0 {
                            from = (byte - b'0') as usize;
                        } else {
                            to = (byte - b'0') as usize;
                        }
                    }
                    _ => in_num = false,
                }
            }
            for _ in 0..moves {
                moved_crates.push(stacks[from - 1].pop_back().unwrap());
            }
            for _ in 0..moves {
                stacks[to - 1].push_back(moved_crates.pop().unwrap());
            }
        }
    }
    rprintln!(
        "d05b: {}",
        stacks
            .into_iter()
            .map(|stack| *stack.back().unwrap() as char)
            .collect::<String>()
    );
}
