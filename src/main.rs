#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buf;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;
use crate::vga_buf::{Alignment, DEFAULT_COLOR, Printer};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let mut screen = Printer::new(Color::Yellow, Alignment::Right);

    for i in 0..100{
        write!(screen, "number {}\n", i);
    }

    loop {}
}