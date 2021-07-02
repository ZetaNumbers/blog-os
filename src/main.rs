#![no_std]
#![no_main]

mod memory_maps;
mod types;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");

    loop {}
}

#[panic_handler]
#[allow(unreachable_code)]
#[no_panic::no_panic]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    println!("{}", panic_info);
    loop {}
}
