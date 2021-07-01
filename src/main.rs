#![no_std]
#![no_main]

mod memory_maps;
mod types;
mod vga;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = &*vga::GLOBAL_VGA_WRITER;
    writeln!(&mut writer, "Hello, World!").unwrap();

    loop {}
}
