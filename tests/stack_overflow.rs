#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use blog_os::{qemu, serial_print};
use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    blog_os::gdt::init();
    init_test_idt();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    let v = 0;
    volatile::Volatile::new(&v).read(); // prevent tail recursion optimizations
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::panic::test_panic_handler(info)
}

static TEST_IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
        idt.double_fault
            .set_handler_fn(test_double_fault_handler)
            .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
});

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_fault_handler: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_print!("[ok]");
    qemu::exit(qemu::ExitCode::Success)
}

pub fn init_test_idt() {
    TEST_IDT.load();
}
