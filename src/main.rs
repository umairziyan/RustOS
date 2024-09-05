#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";
// This disables name mangling to ensure that the rust compiler really outputs a function with the
// name _start.
// This function is diverging because the entry point is never called by any function but invoked
// directly by the operating system or bootloader. Instead of returning, the entry point should
// invoke the exit system call of the operating sytem.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}

// Function to call on panic.
// PanicInfo contains the file and line where the panic happened.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
