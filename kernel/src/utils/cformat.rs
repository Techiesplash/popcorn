use alloc::format;
use alloc::string::ToString;
use core::cmp::min;
use core::fmt::{self, Write, LowerExp, Error};
use core::str::from_utf8;

#[macro_export]
macro_rules! sprintf {
    ($buf:expr, $base_str:expr, $($arg:expr),*) => {
        {
            let args: &[&dyn core::fmt::Display] = &[$(&$arg),*];
            let ret = crate::utils::cformat::cfmt(&mut $buf, $base_str, args);//.unwrap();
            let s = from_utf8(&$buf).unwrap();
            s.trim_end_matches('\0')
        }
    };
}



pub fn cfmt(buffer: &mut [u8], format: &str, args: &[&dyn fmt::Display]) -> fmt::Result {
    let mut buffer_writer = BufferWriter::new(buffer);
    let mut format_iter = format.chars().peekable();
    let mut i = 0;

    while let Some(ch) = format_iter.next() {
        if ch != '%' {
            buffer_writer.write_char(ch)?;
        } else {
            match format_iter.next() {
                Some('%') => buffer_writer.write_char('%')?,
                Some('d') | Some('i') => {
                    if let Some(arg) = args.get(i) {
                        write!(buffer_writer, "{}", arg)?;
                    }
                    i += 1;
                },
                Some('s') => {
                    if let Some(arg) = args.get(i) {
                        write!(buffer_writer, "{}", arg)?;
                    }
                    i += 1;
                },
                Some('f') => {
                    if let Some(arg) = args.get(i) {
                        write!(buffer_writer, "{:.*}", 2, arg)?;
                    }
                    i += 1;
                },
                Some('X') => {
                    if let Some(arg) = args.get(i) {
                        let mut width = 0;
                        while let Some(c) = format_iter.peek().and_then(|c| c.to_digit(10)) {
                            width = width * 10 + c;
                            format_iter.next();
                        }

                        // If width is still 0, it means there were no digits after the 'X', so we set it to the default width of 2.
                        if width == 0 {
                            width = 2;
                        }

                        // Manually format the integer in hexadecimal with the specified width.
                        format_hex(&mut buffer_writer, *arg, width as usize)?;
                    }
                    i += 1;
                },

                Some(c) => buffer_writer.write_char(c)?,
                None => break,
            }
        }
    }

    Ok(())
}


fn format_hex<W: Write>(writer: &mut W, value: &dyn fmt::Display, width: usize) -> Result<(), Error> {
    // Format the integer value as hexadecimal with the specified width manually.
    let mut buffer = [0u8; 16]; // The maximum width for u64 in hexadecimal is 16.
    let mut len = 0;

    // Extract the hexadecimal characters and write them to the buffer in reverse order.
    let mut current_value = match value.to_string().parse::<u64>() {
        Ok(v) => v,
        Err(_) => {
            return Err(fmt::Error);
        }
    };

    while current_value > 0 {
        let remainder = current_value % 16;
        buffer[len] = match remainder {
            0..=9 => b'0' + remainder as u8,
            _ => b'A' + (remainder - 10) as u8,
        };
        current_value /= 16;
        len += 1;
    }

    // Calculate the number of leading zeros needed to achieve the desired width (including offset).
    let leading_zeros = width.saturating_sub(len);

    // Write leading zeros for offset.
    for _ in 0..leading_zeros {
        writer.write_char('0')?;
    }

    // Write the hexadecimal characters in reverse order.
    for i in (0..len).rev() {
        writer.write_char(buffer[i] as char)?;
    }

    Ok(())
}


struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    pos: usize,
}

impl<'a> BufferWriter<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        BufferWriter { buffer, pos: 0 }
    }
}

impl<'a> Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining_space = self.buffer.len() - self.pos;

        if bytes.len() > remaining_space {
            return Err(fmt::Error);
        }

        self.buffer[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let remaining_space = self.buffer.len() - self.pos;

        if remaining_space < c.len_utf8() {
            return Err(fmt::Error);
        }

        let encoded = c.encode_utf8(&mut [0; 4]).to_string(); // Store the result in a variable
        self.buffer[self.pos..self.pos + encoded.len()].copy_from_slice(encoded.as_bytes());
        self.pos += encoded.len();

        Ok(())
    }
}

