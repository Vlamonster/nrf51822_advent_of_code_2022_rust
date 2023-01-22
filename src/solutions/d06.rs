use alloc::collections::VecDeque;
use alloc::vec::Vec;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use cortex_m_semihosting::hprintln;

pub fn p1(input: Vec<u8>) {
    hprintln!(
        "d06a: {}",
        input
            .into_iter()
            .fold_while(
                (0usize, VecDeque::new(), 0),
                |(mut hash, mut window, index), d| {
                    window.push_back(d);
                    hash ^= 1 << (d - b'a');
                    if index >= 4 {
                        hash ^= 1 << (window.pop_front().unwrap() - b'a');
                        if hash.count_ones() == 4 {
                            return Done((hash, window, index + 1));
                        }
                    }
                    Continue((hash, window, index + 1))
                }
            )
            .into_inner()
            .2
    );
}

pub fn p2(input: Vec<u8>) {
    hprintln!(
        "d06b: {}",
        input
            .into_iter()
            .fold_while(
                (0usize, VecDeque::new(), 0),
                |(mut hash, mut window, index), d| {
                    window.push_back(d);
                    hash ^= 1 << (d - b'a');
                    if index >= 14 {
                        hash ^= 1 << (window.pop_front().unwrap() - b'a');
                        if hash.count_ones() == 14 {
                            return Done((hash, window, index + 1));
                        }
                    }
                    Continue((hash, window, index + 1))
                }
            )
            .into_inner()
            .2
    );
}
