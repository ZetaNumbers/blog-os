#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::testable::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;

fn main() {
    println!("Hello, World!");
}

#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::panic::vga_panic_handler(info)
}
