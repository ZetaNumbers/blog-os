use crate::{
    qemu, serial_println,
    types::vga::{Color, ColorCode},
    vga,
};

#[allow(unreachable_code)]
pub fn vga_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    vga::GLOBAL_VGA_WRITER.set_color_code(ColorCode::new(Color::LightRed, Color::Black));
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
