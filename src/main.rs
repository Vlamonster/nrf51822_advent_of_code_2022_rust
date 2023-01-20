#![no_main]
#![no_std]

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm::delay;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use lzss::{Lzss, SliceReader, SliceWriter, SliceWriterExact};
use nrf51_hal as hal;
use nrf51_hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 15000; // in bytes

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;
    let mut output = [0; 15000];
    // let bytes = include_bytes!("../inputs/d01c.txt");
    let result = MyLzss::decompress(SliceReader::new(include_bytes!("../inputs/d01c.txt")), SliceWriter::new(&mut output));
    rtt_init_print!();

    for byte in output{
        rprintln!("{}", byte as char);
        delay(16_000_000);
    }

    loop {

    }
}
