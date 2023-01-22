use alloc::string::String;
use alloc::vec::Vec;
use cortex_m_semihosting::hprintln;

pub fn p1(input: Vec<u8>) {
    hprintln!(
        "d04a: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .filter(|pair| {
                let (first, second) = pair.split_once(',').unwrap();
                let (a, b) = first.split_once('-').unwrap();
                let (c, d) = second.split_once('-').unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let c = c.parse::<usize>().unwrap();
                let d = d.parse::<usize>().unwrap();
                (a >= c && b <= d) || (a <= c && b >= d)
            })
            .count()
    );
}

pub fn p2(input: Vec<u8>) {
    hprintln!(
        "d04b: {}",
        input
            .into_iter()
            .map(|d| d as char)
            .collect::<String>()
            .lines()
            .filter(|pair| {
                let (first, second) = pair.split_once(',').unwrap();
                let (a, b) = first.split_once('-').unwrap();
                let (c, d) = second.split_once('-').unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let c = c.parse::<usize>().unwrap();
                let d = d.parse::<usize>().unwrap();
                a <= d && c <= b
            })
            .count()
    );
}
