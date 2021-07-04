#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt, try_trait_v2, try_blocks)]
#![test_runner(crate::testable::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod panic;
pub mod qemu;
pub mod serial;
pub mod testable;
mod util;
pub mod vga;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic::test_panic_handler(info)
}
