use core::fmt::Write;
use spin::{Lazy, Mutex};
use vga::{
    colors::{Color16, TextModeColor},
    writers::{Screen, ScreenCharacter, Text80x25, TextWriter},
};
use x86_64::instructions::interrupts::without_interrupts;

pub static GLOBAL_VGA_WRITER: Lazy<SyncVgaWriter> = Lazy::new(|| {
    SyncVgaWriter(Mutex::new({
        let color_code = TextModeColor::new(Color16::Yellow, Color16::Black);
        let text_mode = TextMode::new();
        text_mode.clear_screen();

        VgaWriter {
            column: 0,
            color_code,
            buffer: [[ScreenCharacter::new(b' ', color_code); TextMode::WIDTH]; TextMode::HEIGHT],
            text_mode,
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
pub fn _print(args: core::fmt::Arguments) {
    let mut write = &*GLOBAL_VGA_WRITER;
    write.write_fmt(args).unwrap();
}

pub struct SyncVgaWriter(Mutex<VgaWriter>);

impl SyncVgaWriter {
    pub fn color_code(&self) -> TextModeColor {
        without_interrupts(|| self.0.lock().color_code)
    }

    pub fn set_color_code(&self, color: TextModeColor) {
        without_interrupts(|| self.0.lock().color_code = color);
    }
}

impl Write for &'_ SyncVgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        without_interrupts(|| self.0.lock().write_str(s))
    }
}

type TextMode = Text80x25;

pub struct VgaWriter {
    /// Less than Screen::WIDTH
    column: usize,
    pub color_code: TextModeColor,
    buffer: Buffer,
    text_mode: TextMode,
}

pub type Buffer = [[ScreenCharacter; TextMode::WIDTH]; TextMode::HEIGHT];

impl VgaWriter {
    fn flush(&mut self) {
        let (_guard, frame_buffer) = self.text_mode.get_frame_buffer();
        unsafe { (frame_buffer as *mut Buffer).write_volatile(self.buffer) };
    }

    fn load(&mut self) {
        let (_guard, frame_buffer) = self.text_mode.get_frame_buffer();
        self.buffer = unsafe { (frame_buffer as *mut Buffer).read_volatile() };
    }

    fn new_line(&mut self) {
        self.buffer.copy_within(1..TextMode::HEIGHT, 0);
        self.buffer[TextMode::HEIGHT - 1].fill(ScreenCharacter::new(b' ', self.color_code));
        self.column = 0;
    }

    fn write_byte(&mut self, byte: u8) {
        if self.column >= TextMode::WIDTH {
            self.new_line()
        }
        *unsafe { self.buffer[TextMode::HEIGHT - 1].get_unchecked_mut(self.column) } =
            ScreenCharacter::new(byte, self.color_code);
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
    let color_code = TextModeColor::new(Color16::White, Color16::Black);
    GLOBAL_VGA_WRITER.set_color_code(color_code);
    let s = "Some test string that fits on a single line";
    println!("\n{}", s);

    let mut buf = [[ScreenCharacter::new(b' ', color_code); TextMode::WIDTH]; 2];

    for (i, o) in s
        .bytes()
        .map(|byte| ScreenCharacter::new(byte, color_code))
        .zip(buf.iter_mut().flatten())
    {
        *o = i;
    }

    without_interrupts(|| {
        let mut writer_guard = GLOBAL_VGA_WRITER.0.lock();
        writer_guard.load();
        assert_eq!(writer_guard.buffer[TextMode::HEIGHT - 2..], buf)
    })
}
