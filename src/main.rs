#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

// This disables name mangling to ensure that the rust compiler really outputs a function with the
// name _start.
// This function is diverging because the entry point is never called by any function but invoked
// directly by the operating system or bootloader. Instead of returning, the entry point should
// invoke the exit system call of the operating sytem.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

// Function to call on panic.
// PanicInfo contains the file and line where the panic happened.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
