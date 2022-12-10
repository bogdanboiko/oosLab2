pub const BUF_HEIGHT: u32 = 25;
pub const BUF_WIDTH: u32 = 80;
pub const DEFAULT_COLOR: u8 = (COLOR_BLACK << 4) | COLOR_LIGHT_GREEN;

const BUF_START: u32 = 0xb8000;
const COLOR_LIGHT_GREEN: u8 = 0xa;
const COLOR_BLACK: u8 = 0x0;

pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

pub enum Alignment {
    Left,
    Right,
    Center,
}

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8,
}

pub struct Printer {
    buffer: *mut u8,
    column: u32,
    row: u32,
    color: u8,
    align: Alignment
}

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Printer {
    pub fn new(color: Color, _align: Alignment) -> Printer {
        return Printer {
            buffer: BUF_START as *mut u8,
            color: ((color as u8) << 4) | (DEFAULT_COLOR as u8),
            row: 0,
            column: Self::calculate_column_position(&_align),
            align: _align,
        };
    }

    pub fn print_hello_world(&mut self) {
        let mut i = 0;
        for byte in "Hello world!".bytes() {
            self.write_symbol(i, AsciiChar { char_byte: byte, color_byte: self.color });
            i += 1;
        }
    }

    pub fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                if self.row < BUF_HEIGHT - 1 {
                    self.row += 1;
                } else {
                    self.move_down();
                }

                self.column = Self::calculate_column_position(&self.align);
            } else {
                match self.align {
                    Alignment::Left => {
                        self.left_right_append(byte);
                        self.column += 1;
                    }

                    Alignment::Right => {
                        self.move_left();
                        self.left_right_append(byte)
                    }

                    Alignment::Center => {
                        self.center_append(byte);
                        self.column += 1;
                    }
                }
            }
        }
    }

    pub fn left_right_append(&self, symbol: u8) {
        let target_offset = self.row * BUF_WIDTH + self.column;

        self.write_symbol(target_offset, AsciiChar { char_byte: symbol, color_byte: self.color });
    }

    pub fn center_append(&self, symbol: u8) {
        if self.column % 2 != 0 {
            self.move_left();
        }

        let target_offset = self.row * BUF_WIDTH + BUF_WIDTH / 2 + self.column / 2;

        self.write_symbol(target_offset, AsciiChar { char_byte: symbol, color_byte: self.color });
    }

    pub fn calculate_column_position(align: &Alignment) -> u32 {
        match align {
            Alignment::Left => 0,
            Alignment::Right => BUF_WIDTH - 1,
            Alignment::Center => 0
        }
    }

    pub fn move_down(&self) {
        for i in 1..BUF_HEIGHT {
            for j in 0..BUF_WIDTH {
                self.write_symbol((i - 1) * BUF_WIDTH + j, self.read_symbol(i * BUF_WIDTH + j));
            }
        }

        for i in 0..BUF_WIDTH {
            self.write_symbol((BUF_HEIGHT - 1) * BUF_WIDTH + i, AsciiChar { char_byte: b' ', color_byte: DEFAULT_COLOR })
        }
    }

    pub fn move_left(&self) {
        for i in 1..BUF_WIDTH {
            self.write_symbol(self.row * BUF_WIDTH + i - 1, self.read_symbol(self.row * BUF_WIDTH + i));
        }
    }

    pub fn write_symbol(&self, offset: u32, char: AsciiChar) {
        unsafe {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
        }
    }

    pub fn read_symbol(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar {
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1),
            };
        }
    }
}