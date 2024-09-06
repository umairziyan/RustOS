#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod serial;
mod vga_buffer;

// This disables name mangling to ensure that the rust compiler really outputs a function with the
// name _start.
// This function is diverging because the entry point is never called by any function but invoked
// directly by the operating system or bootloader. Instead of returning, the entry point should
// invoke the exit system call of the operating sytem.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Conditional compliation to add the call to test_main only in test contexts.
    #[cfg(test)]
    test_main();

    loop {}
}

// Function to call on panic.
// PanicInfo contains the file and line where the panic happened.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // exit if successful
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion...");
    assert_eq!(1, 0);
    serial_println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // iobase of trhe isa-debug-exit device.
        let mut port = Port::new(0xf4);

        // Write exit code to the port.
        port.write(exit_code as u32);
    }
}
