#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(iter_array_chunks)]
#![feature(vec_push_within_capacity)]
#![feature(vec_into_raw_parts)]

mod solutions;

extern crate alloc;

use crate::solutions::{
    d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19,
    d20, d21, d22, d23, d24, d25,
};
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::slice;
use cortex_m::asm::delay;
use lz4_flex::decompress_into;
use nrf51_hal::pac::timer0::bitmode::BITMODE_A;
use nrf51_hal::pac::timer0::mode::MODE_A;
use nrf51_hal::pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

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

type Solution = fn(&mut [u8], &[u8]);

const SOLUTIONS: [[Solution; 2]; 25] = [
    [d01::p1, d01::p2],
    [d02::p1, d02::p2],
    [d03::p1, d03::p2],
    [d04::p1, d04::p2],
    [d05::p1, d05::p2],
    [d06::p1, d06::p2],
    [d07::p1, d07::p2],
    [d08::p1, d08::p2],
    [d09::p2, d09::p2],
    [d10::p1, d10::p2],
    [d11::p1, d11::p2],
    [d12::p1, d12::p2],
    [d13::p1, d13::p2],
    [d14::p1, d14::p2],
    [d15::p1, d15::p2],
    [d16::p1, d16::p2],
    [d17::p1, d17::p2],
    [d18::p1, d18::p2],
    [d19::p1, d19::p2],
    [d20::p1, d20::p2],
    [d21::p1, d21::p2],
    [d22::p1, d22::p2],
    [d23::p1, d23::p2],
    [d24::p1, d24::p2],
    [d25::p1, d25::p2],
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

fn solve(d: usize, p: usize) {
    let mut memory = [0u8; 28000];
    let input_size = decompress_into(INPUTS[d - 1], &mut memory).unwrap();
    let input = unsafe { slice::from_raw_parts(memory.as_mut_ptr(), input_size) };
    SOLUTIONS[d - 1][p - 1](&mut memory, input);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let tim0 = Peripherals::take().unwrap().TIMER0;
    tim0.mode.write(|w| w.mode().variant(MODE_A::TIMER));
    tim0.bitmode
        .write(|w| w.bitmode().variant(BITMODE_A::_32BIT));
    tim0.tasks_start.write(|w| unsafe { w.bits(1) });

    rprintln!("Starting computation:");

    // #[derive(Debug, Copy, Clone)]
    // #[repr(packed)]
    // struct Toople {
    //     a: u16,
    //     b: u16,
    // }
    //
    // let mut x = [1u8; 28000];
    // let pointer = x.as_mut_ptr().cast::<Toople>();
    // let y = unsafe {
    //     Vec::from_raw_parts(pointer, 10, 10)
    // };
    // rprintln!("{:?}", y);
    // drop(x);
    // // rprintln!("h");

    for d in 1..=25 {
        for p in 1..=2 {
            solve(d, p);
        }
    }

    tim0.tasks_capture[0].write(|w| unsafe { w.bits(1) });
    rprintln!("computation took {:?}us", tim0.cc[0].read().bits());

    loop {
        rprintln!("beep");
        delay(16_000_000);
    }
}
