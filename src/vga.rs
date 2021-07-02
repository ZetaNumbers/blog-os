use core::fmt::Write;

use spin::{Lazy, Mutex};

use crate::memory_maps::vga as vga_mmap;
use crate::types::vga::*;

pub static GLOBAL_VGA_WRITER: Lazy<SyncVgaWriter> = Lazy::new(|| {
    SyncVgaWriter(Mutex::new({
        let color_code = ColorCode::new(Color::White, Color::Black);
        VgaWriter {
            pos: 0,
            color_code,
            buffer: [ScreenChar::empty(color_code); vga_mmap::BUFFER_SIZE],
            memory: unsafe { &mut *vga_mmap::BUFFER_PTR },
        }
    }))
});

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
    /// Less than BUFFER_SIZE
    pos: usize,
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

    fn new_line(&mut self) {
        let new_pos_unchecked = (self.pos / vga_mmap::BUFFER_WIDTH + 1) * vga_mmap::BUFFER_WIDTH;

        unsafe { self.buffer.get_unchecked_mut(self.pos..new_pos_unchecked) }
            .fill(ScreenChar::empty(self.color_code));

        self.pos = new_pos_unchecked % vga_mmap::BUFFER_SIZE;
    }

    fn write_byte(&mut self, byte: u8) {
        *unsafe { self.buffer.get_unchecked_mut(self.pos) } = ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        };

        self.pos = (self.pos + 1) % vga_mmap::BUFFER_SIZE
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
