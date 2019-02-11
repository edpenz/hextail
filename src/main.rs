use std::fmt;
use std::io::{self, Read, Write};

struct VtColor {
    vt_code: u8,
}

impl fmt::Display for VtColor {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "\x1b[{}m", self.vt_code)
    }
}

impl VtColor {
    const DEFAULT: VtColor = VtColor { vt_code: 39 };
    const RED: VtColor = VtColor { vt_code: 31 };
    const MAGENTA: VtColor = VtColor { vt_code: 35 };
    const GREY: VtColor = VtColor { vt_code: 90 };
}

fn color_for_ascii(ascii_code: &u8) -> VtColor {
    match ascii_code {
        0x00 => VtColor::RED,            // Null
        0x01...0x20 => VtColor::MAGENTA, // Control codes
        0x7f...0xff => VtColor::GREY,    // Extended ASCII
        _ => VtColor::DEFAULT,           // Printable ASCII
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut buffer = [0 as u8; 16];
    let mut offset: usize = 0;

    loop {
        // Read input.
        let read_count = {
            let read_offset = offset % buffer.len();
            match stdin.read(&mut buffer[read_offset..]) {
                Err(_) | Ok(0) => break,
                Ok(n) => n,
            }
        };

        // Calculate some helpful state.
        let old_offset = offset;
        offset = old_offset + read_count;

        let line_offset = old_offset / 16 * 16;

        let is_line_dirty = old_offset % 16 != 0;
        let is_line_finished = offset % 16 == 0;

        // Reset cursor position to redraw line.
        if is_line_dirty {
            write!(stdout, "\r").unwrap()
        };

        // Print line offset as hex: "XXXXXXXX  "
        write!(stdout, "{}{:08x}  ", VtColor::DEFAULT, line_offset).unwrap();

        // Print line bytes as hex: "XX XX XX XX XX XX XX XX  XX XX XX XX XX XX XX XX  "
        for (i, byte) in buffer[..offset - line_offset].iter().enumerate() {
            match i {
                7 | 15 => write!(stdout, "{}{:02x}  ", color_for_ascii(byte), byte).unwrap(),
                _ => write!(stdout, "{}{:02x} ", color_for_ascii(byte), byte).unwrap(),
            }
        }
        for i in (offset - line_offset)..16 {
            match i {
                7 | 15 => write!(stdout, "    ").unwrap(), // Blank for "XX  "
                _ => write!(stdout, "   ").unwrap(),       // Blank for "XX "
            }
        }

        // Print line bytes as ASCII: "|AAAAAAAAAAAAAAAA|"
        write!(stdout, "{}|", VtColor::DEFAULT).unwrap();
        for byte in buffer[..offset - line_offset].iter() {
            match byte {
                0x20...0x7e => write!(stdout, "{}{}", color_for_ascii(byte), *byte as char).unwrap(),
                _ => write!(stdout, "{}.", color_for_ascii(byte)).unwrap(),
            }
        }
        write!(stdout, "{}|", VtColor::DEFAULT).unwrap();

        // Wrap to next line if done with this one.
        if is_line_finished {
            write!(stdout, "\n").unwrap()
        }

        stdout.flush().unwrap();
    }

    println!();
}
