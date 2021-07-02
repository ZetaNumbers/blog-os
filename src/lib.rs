#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testable::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod memory_maps;
pub mod panic;
pub mod qemu;
pub mod serial;
pub mod testable;
mod types;
pub mod vga;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic::test_panic_handler(info)
}
