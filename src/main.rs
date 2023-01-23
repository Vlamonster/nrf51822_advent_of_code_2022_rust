#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

mod solutions;

extern crate alloc;

use crate::solutions::{d01, d02, d03, d04, d05, d06, d07, d08};
use alloc::vec;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::asm::delay;
use lzss::{Lzss, SliceReader, SliceWriter};
use nrf51_hal as hal;
use nrf51_hal::pac::Peripherals;
use nrf51_hal::pac::timer0::mode::MODE_A;
use nrf51_hal::pac::timer2::bitmode::BITMODE_A;
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

fn read_file(n: usize) -> Vec<u8> {
    let mut output = vec![0; 27000];
    let result =
        MyLzss::decompress(SliceReader::new(INPUTS[n]), SliceWriter::new(&mut output)).unwrap();
    drop(output);
    let mut output = vec![0; result];
    MyLzss::decompress(SliceReader::new(INPUTS[n]), SliceWriter::new(&mut output)).unwrap();
    output
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let tim0 = Peripherals::take().unwrap().TIMER0;
    tim0.mode.write(|w| w.mode().variant(MODE_A::TIMER));
    tim0.bitmode.write(|w| w.bitmode().variant(BITMODE_A::_32BIT));
    tim0.tasks_start.write(|w| unsafe { w.bits(1) });

    rprintln!("Starting computation:");

    d01::p1(read_file(0));
    d01::p2(read_file(0));
    d02::p1(read_file(1));
    d02::p2(read_file(1));
    d03::p1(read_file(2));
    d03::p2(read_file(2));
    d04::p1(read_file(3));
    d04::p2(read_file(3));
    d05::p1(read_file(4));
    d05::p2(read_file(4));
    d06::p1(read_file(5));
    d06::p2(read_file(5));
    d07::p1(read_file(6));
    d07::p2(read_file(6));
    d08::p1(read_file(7));
    d08::p2(read_file(7));

    tim0.tasks_capture[0].write(|w| unsafe { w.bits(1) });
    rprintln!("computation took {:?}us",tim0.cc[0].read().bits());

    loop {
        rprintln!("beep");
        delay(16_000_000);
    }
}
