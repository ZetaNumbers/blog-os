use core::fmt;
use core::fmt::Write;

use spin::{Lazy, Mutex};

use crate::memory_maps::vga as vga_mmap;
use crate::types::vga::*;

pub static GLOBAL_VGA_WRITER: Lazy<SyncVgaWriter> = Lazy::new(|| {
    SyncVgaWriter(Mutex::new({
        let color_code = ColorCode::new(Color::White, Color::Black);
        VgaWriter {
            column: 0,
            color_code,
            buffer: [[ScreenChar::empty(color_code); vga_mmap::BUFFER_WIDTH];
                vga_mmap::BUFFER_HEIGHT],
            memory: unsafe { &mut *vga_mmap::BUFFER_PTR },
        }
    }))
});

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut write = &*GLOBAL_VGA_WRITER;
    write.write_fmt(args).unwrap();
}

pub struct SyncVgaWriter(Mutex<VgaWriter>);

impl SyncVgaWriter {
    #[allow(dead_code)]
    pub fn set_color_code(&self, cc: ColorCode) {
        self.0.lock().color_code = cc;
    }
}

impl Write for &'_ SyncVgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.lock().write_str(s)
    }
}

pub struct VgaWriter {
    /// Less than BUFFER_WIDTH
    column: usize,
    pub color_code: ColorCode,
    buffer: vga_mmap::Buffer,
    memory: *mut vga_mmap::Buffer,
}

unsafe impl Send for VgaWriter {}
unsafe impl Sync for VgaWriter {}

impl VgaWriter {
    fn flush(&mut self) {
        unsafe { self.memory.write_volatile(self.buffer) };
    }

    /// Volatile load VGA text buffer
    #[cfg(test)]
    fn load(&mut self) {
        self.buffer = unsafe { self.memory.read_volatile() };
    }

    fn new_line(&mut self) {
        self.buffer.copy_within(1..vga_mmap::BUFFER_HEIGHT, 0);
        self.buffer[vga_mmap::BUFFER_HEIGHT - 1].fill(ScreenChar::empty(self.color_code));
        self.column = 0;
    }

    fn write_byte(&mut self, byte: u8) {
        if self.column >= vga_mmap::BUFFER_WIDTH {
            self.new_line()
        }
        *unsafe { self.buffer[vga_mmap::BUFFER_HEIGHT - 1].get_unchecked_mut(self.column) } =
            ScreenChar {
                ascii_character: byte,
                color_code: self.color_code,
            };

        self.column += 1;
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if s.len() == 0 {
            return Ok(());
        }

        for b in s.bytes() {
            match b {
                b'\n' => self.new_line(),
                b @ 0x20..=0x7e => self.write_byte(b),
                _ => self.write_byte(0xfe),
            }
        }

        self.flush();
        Ok(())
    }
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let color_code = ColorCode::new(Color::White, Color::Black);
    GLOBAL_VGA_WRITER.set_color_code(color_code);
    let s = "Some test string that fits on a single line";
    println!("\n{}", s);

    let mut writer_guard = GLOBAL_VGA_WRITER.0.lock();
    let mut buf = [[ScreenChar::empty(color_code); vga_mmap::BUFFER_WIDTH]; 2];

    for (i, o) in s
        .bytes()
        .map(|b| ScreenChar {
            ascii_character: b,
            color_code,
        })
        .zip(buf.iter_mut().flatten())
    {
        *o = i;
    }

    writer_guard.load();
    assert_eq!(writer_guard.buffer[vga_mmap::BUFFER_HEIGHT - 2..], buf)
}
