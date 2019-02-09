use std::io::{self, Read, Write};

fn color_default() -> &'static str {
    "\x1b[39m"
}

fn color_lookup(ascii_code: &u8) -> &'static str {
    match ascii_code {
        0x00 => "\x1b[31m",        // Null is red
        0x01...0x20 => "\x1b[35m", // Control codes are magenta
        0x7f...0xff => "\x1b[90m", // Extended ASCII is grey
        _ => "\x1b[39m",           // Printable ASCII is default
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
        write!(stdout, "{}{:08x}  ", color_default(), line_offset).unwrap();

        // Print line bytes as hex: "XX XX XX XX XX XX XX XX  XX XX XX XX XX XX XX XX  "
        for (i, byte) in buffer[..offset - line_offset].iter().enumerate() {
            match i {
                7 | 15 => write!(stdout, "{}{:02x}  ", color_lookup(byte), byte).unwrap(),
                _ => write!(stdout, "{}{:02x} ", color_lookup(byte), byte).unwrap(),
            }
        }
        for i in (offset - line_offset)..16 {
            match i {
                7 | 15 => write!(stdout, "    ").unwrap(),
                _ => write!(stdout, "   ").unwrap(),
            }
        }

        // Print line bytes as ASCII: "|AAAAAAAAAAAAAAAA|"
        write!(stdout, "{}|", color_default()).unwrap();
        for byte in buffer[..offset - line_offset].iter() {
            match byte {
                0x20...0x7e => write!(stdout, "{}{}", color_lookup(byte), *byte as char).unwrap(),
                _ => write!(stdout, "{}.", color_lookup(byte)).unwrap(),
            }
        }
        write!(stdout, "{}|", color_default()).unwrap();

        // Wrap to next line if done with this one.
        if is_line_finished {
            write!(stdout, "\n").unwrap()
        }

        stdout.flush().unwrap();
    }

    println!();
}
