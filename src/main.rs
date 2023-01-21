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

    // initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;

    let mut output = [0; 27000];
    let result = MyLzss::decompress(
        SliceReader::new(include_bytes!("../inputs/d01c.txt")),
        SliceWriter::new(&mut output),
    );
    d01::p1(&output);
    d01::p2(&output);

    loop {
        rprintln!("beep");
        delay(16_000_000);
    }
}
