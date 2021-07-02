use crate::{qemu, serial_println};
use no_panic::no_panic;

#[allow(unreachable_code)]
#[no_panic]
pub fn vga_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    crate::println!("{}", panic_info);
    loop {}
}

#[allow(unreachable_code)]
#[no_panic]
pub fn test_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    use crate::{qemu, serial_println};
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", panic_info);
    qemu::exit(qemu::ExitCode::Failed)
}

#[allow(unreachable_code)]
#[no_panic]
pub fn fail_test_panic_handler(_: &core::panic::PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}
