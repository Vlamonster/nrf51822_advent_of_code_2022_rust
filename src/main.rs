#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

mod solutions;

extern crate alloc;

use crate::solutions::d01;
use alloc::vec;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::asm::delay;
use lzss::{Lzss, SliceReader, SliceWriter};
use nrf51_hal as hal;
use rtt_target::{rprintln, rtt_init_print};

type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;

const INPUTS: [&[u8]; 25] = [
    include_bytes!("../inputs/d01c.txt"),
    include_bytes!("../inputs/d02c.txt"),
    include_bytes!("../inputs/d03c.txt"),
    include_bytes!("../inputs/d04c.txt"),
    include_bytes!("../inputs/d05c.txt"),
    include_bytes!("../inputs/d06c.txt"),
    include_bytes!("../inputs/d07c.txt"),
    include_bytes!("../inputs/d08c.txt"),
    include_bytes!("../inputs/d09c.txt"),
    include_bytes!("../inputs/d10c.txt"),
    include_bytes!("../inputs/d11c.txt"),
    include_bytes!("../inputs/d12c.txt"),
    include_bytes!("../inputs/d13c.txt"),
    include_bytes!("../inputs/d14c.txt"),
    include_bytes!("../inputs/d15c.txt"),
    include_bytes!("../inputs/d16c.txt"),
    include_bytes!("../inputs/d17c.txt"),
    include_bytes!("../inputs/d18c.txt"),
    include_bytes!("../inputs/d19c.txt"),
    include_bytes!("../inputs/d20c.txt"),
    include_bytes!("../inputs/d21c.txt"),
    include_bytes!("../inputs/d22c.txt"),
    include_bytes!("../inputs/d23c.txt"),
    include_bytes!("../inputs/d24c.txt"),
    include_bytes!("../inputs/d25c.txt"),
];

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 27000; // in bytes

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        rprintln!("panic");
        cortex_m::asm::bkpt();
    }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    loop {
        rprintln!("alloc error: {:?}", layout);
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    let mut output = vec![0; 27000];
    let result =
        MyLzss::decompress(SliceReader::new(INPUTS[0]), SliceWriter::new(&mut output)).unwrap();
    drop(output);
    let mut output = vec![0; result];
    MyLzss::decompress(SliceReader::new(INPUTS[0]), SliceWriter::new(&mut output)).unwrap();
    d01::p1(output);
    // d01::p2(&output);

    loop {
        rprintln!("beep");
        delay(16_000_000);
    }
}
