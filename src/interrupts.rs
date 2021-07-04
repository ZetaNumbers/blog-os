use crate::{gdt, print, println, util::WildcardTry};
use core::{fmt::Debug, panic};
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Lazy;
use x86_64::{
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
    VirtAddr,
};

pub fn init_idt() {
    IDT.load();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
    idt
});

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!(
        "{:#?}",
        CpuExceptionError {
            exception: "breakpoint",
            stack_frame,
            optional_error_code: (),
            additional_info: ()
        }
    );
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "{:#?}",
        CpuExceptionError {
            exception: "double fault",
            stack_frame,
            optional_error_code: error_code,
            additional_info: ()
        }
    );
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    panic!(
        "{:#?}",
        CpuExceptionError {
            exception: "page fault",
            optional_error_code: error_code,
            stack_frame,
            additional_info: AccessedAddress(Cr2::read())
        }
    );

    #[derive(Debug)]
    struct AccessedAddress(VirtAddr);
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let scancode: u8 = unsafe { Port::new(0x60).read() };
    let mut keyboard = KEYBOARD.lock();
    (|| {
        let keyevent = keyboard.add_byte(scancode)??;
        let key = keyboard.process_keyevent(keyevent)?;
        match key {
            pc_keyboard::DecodedKey::RawKey(key) => print!("{:?}", key),
            pc_keyboard::DecodedKey::Unicode(ch) => print!("{}", ch),
        }
        WildcardTry
    })();

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }

    static KEYBOARD: spin::Lazy<spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> =
        spin::Lazy::new(|| {
            spin::Mutex::new(Keyboard::new(
                layouts::Us104Key,
                ScancodeSet1,
                HandleControl::Ignore,
            ))
        });
}

#[derive(Debug)]
struct CpuExceptionError<E: Debug, I: Debug> {
    exception: &'static str,
    stack_frame: InterruptStackFrame,
    optional_error_code: E,
    additional_info: I,
}
