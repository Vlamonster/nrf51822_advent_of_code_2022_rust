#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use alloc_cortex_m::CortexMHeap;
use cortex_m::asm::delay;
use lzss::{Lzss, SliceReader, SliceWriter};
use nrf51_hal as hal;
use rtt_target::{rprint, rtt_init_print};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 27000; // in bytes

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    // initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    type MyLzss = Lzss<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>;

    let mut output = vec![0; 27000];

    // let bytes = include_bytes!("../inputs/d01c.txt");

    let _result = MyLzss::decompress(
        SliceReader::new(include_bytes!("../inputs/compressed_input.txt")),
        SliceWriter::new(output.as_mut_slice()),
    );

    for byte in output {
        rprint!("{}", byte as char);
        delay(1000);
    }

    loop {}
}
