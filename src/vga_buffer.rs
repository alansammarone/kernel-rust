use core::fmt;
use lazy_static::lazy_static;
use spin::mutex::SpinMutex;
use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
        // We need to build a 1-byte bit array, the first 4 bits of which
        // represents the foreground color (including the last, bright flag)
        // the next 3 the background color, and the last one the blink flag
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    // Builds a 2D array with shape (WIDTH, HEIGHT)
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    initialized: bool,
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    // TODO: for now, color_code is a writer (or screen)-level property, since it is specified here
    // and passed unmodified to all write_byte calls. This implies different bytes
    // can't have different `color_code`'s. For now, we just set _all_ bytes
    // in the buffer to use this color code in the beginning.
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        if self.initialized == false {
            self.fill_buffer();
            self.initialized = true;
        }

        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let (row, col) = (BUFFER_HEIGHT - 1, self.column_position);
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
            self.column_position = 0;
        }
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    pub fn fill_buffer(&mut self) {
        // TODO: better name. buffer is an implementation detail
        // TODO figure out why cursor is blinking when calling this method
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(blank);
            }
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// A lock is required here to avoid race conditions.
// OS does not provide enough functionality, yet, to allow us
// to use the std Mutex. We therefore use a spinlock for the time being.
lazy_static! {
    pub static ref WRITER: SpinMutex<Writer> = SpinMutex::new(Writer {
        initialized: false,
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::LightGray),
        // color_code: ColorCode::new(Color::White, Color::Black),
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

#[test_case]
fn test_println_clears_line() {
    println!("Print some string");
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    for i in 0..200 {
        println!("test_println_many output {}", i);
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some long-but-single-line text";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
