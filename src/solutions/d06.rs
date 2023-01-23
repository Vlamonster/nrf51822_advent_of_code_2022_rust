use alloc::collections::VecDeque;
use alloc::vec::Vec;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use rtt_target::rprintln;
use nrf51_hal::pac::Peripherals;
use nrf51_hal::pac::timer0::mode::MODE_A;
use nrf51_hal::pac::timer2::bitmode::BITMODE_A;

/// Measured speed: 6232us.
pub fn p1(input: Vec<u8>) {
    let mut hash = 0u32;
    let mut counter = 0;
    let mut window = [0; 4];
    for byte in input {
        if counter >= 4 {
            hash ^= 1 << (window[counter & 0b11] - b'a');
        }
        window[counter & 0b11] = byte;
        hash ^= 1 << (byte - b'a');
        counter += 1;

        if hash.count_ones() == 4 {
            break;
        }
    }
    rprintln!("d06a: {}", counter);
}

/// Measured speed: 11267us.
pub fn p2(input: Vec<u8>) {
    let mut hash = 0u32;
    let mut counter = 0;
    let mut window_index = 0;
    let mut window = [0; 14];
    for byte in input {
        if counter >= 14 {
            if window_index == 14 {
                window_index = 0;
            }
            hash ^= 1 << (window[window_index] - b'a');
        }
        window[window_index] = byte;
        window_index += 1;
        hash ^= 1 << (byte - b'a');
        counter += 1;
        if hash.count_ones() == 14 {
            break;
        }
    }
    rprintln!("d06b: {}", counter);
}
