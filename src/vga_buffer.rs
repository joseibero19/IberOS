use core::fmt;
use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

// VGA text buffer colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear_screen() {
    for row in 0..BUFFER_HEIGHT {
        WRITER.lock().clear_row(row);
    }
    WRITER.lock().column_position = 0;
}

pub fn print_logo() {
    let mut writer = WRITER.lock();

    // Set color to bright cyan for the logo with black background
    let original_color = writer.color_code;
    writer.color_code = ColorCode::new(Color::LightCyan, Color::Black);

    // Add some space at the top
    writer.write_string("\n\n");

    // ASCII art for iberOS logo
    writer.write_string("    _  _                 ___  ___ \n");
    writer.write_string("   (_)| |               / _ \\/ __|\n");
    writer.write_string("    _ | |__   ___  _ __| | | \\__ \\\n");
    writer.write_string("   | || '_ \\ / _ \\| '__| | | |__) |\n");
    writer.write_string("   | || |_) |  __/| |  | |_| / __/\n");
    writer.write_string("   |_||_.__/ \\___||_|   \\___/\\___|\n");

    // Add a tagline
    writer.write_string("\n      A Rust Microkernel OS\n\n");

    // Reset color
    writer.color_code = original_color;
}

// Function to print text centered on the screen with a specific color
pub fn print_centered(text: &str, color: Color) {
    let mut writer = WRITER.lock();
    let original_color = writer.color_code;

    // Set the color
    writer.color_code = ColorCode::new(color, Color::Black);

    // Calculate padding to center the text
    let padding = (BUFFER_WIDTH.saturating_sub(text.len())) / 2;

    // Move to the beginning of the current line
    writer.column_position = 0;

    // Add padding
    for _ in 0..padding {
        writer.write_byte(b' ');
    }

    // Write the text
    writer.write_string(text);

    // Add a newline
    writer.write_byte(b'\n');

    // Reset color
    writer.color_code = original_color;
}
