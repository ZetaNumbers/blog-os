use crate::{qemu, serial_println, vga::GLOBAL_VGA_WRITER};
use vga::colors::{Color16, TextModeColor};

#[allow(unreachable_code)]
pub fn vga_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    GLOBAL_VGA_WRITER.set_color_code(TextModeColor::new(Color16::LightRed, Color16::Black));
    crate::print!("{}", panic_info);
    crate::hlt_loop()
}

#[allow(unreachable_code)]
pub fn test_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", panic_info);
    qemu::exit(qemu::ExitCode::Failed)
}

#[allow(unreachable_code)]
pub fn fail_test_panic_handler(_: &core::panic::PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success)
}
