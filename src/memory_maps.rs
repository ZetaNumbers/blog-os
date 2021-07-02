//! Allow only one reference per one magic pointers
//! Define all magic pointers there

/// Referenced in [crate::vga]
pub mod vga {
    use crate::types::vga::ScreenChar;

    pub const BUFFER_PTR: *mut Buffer = 0xb8000 as *mut _;
    pub type Buffer = [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT];
    pub const BUFFER_WIDTH: usize = 80;
    pub const BUFFER_HEIGHT: usize = 25;
}
