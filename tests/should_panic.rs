#![no_std]
#![no_main]

use blog_os::{
    qemu::{exit, ExitCode},
    serial_print, serial_println,
};

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit(ExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::panic::fail_test_panic_handler(info)
}
