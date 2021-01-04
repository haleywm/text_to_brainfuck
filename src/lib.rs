use std::io::{BufWriter, Cursor, Read, Write};

//const IN_ERR: &str = "Error reading input";
//const OUT_ERR: &str = "Error writing output";

pub fn parse(input: impl Read, output: impl Write, line_len: usize) {
    let mut out = BufWriter::new(output);
    let mut line_buf = Cursor::new(vec![0; 256]);
    // Tracks how much has been written since last new line
    let mut last_written = 0;
    // The current value of memory 0
    let mut cur = 0;
    for letter in input.bytes() {
        let letter = letter.unwrap();

        let (pos, dif) = if letter >= cur {
            (true, letter - cur)
        } else {
            (false, cur - letter)
        };

        if dif != 0 {
            // Lets see what we gotta do
            let (a, b, mut rem) = fact_and_left(dif as usize);
            if !pos {
                // Rem goes other way
                rem *= -1;
            }
            if a == 1 {
                // No need to do extra syntax
                if pos {
                    rem += b as i64;
                } else {
                    rem -= b as i64;
                }
            } else if b == 1 {
                // No need to do extra syntax
                if pos {
                    rem += a as i64;
                } else {
                    rem -= a as i64;
                }
            } else {
                line_buf.write(b">").unwrap();
                for _ in 0..a {
                    line_buf.write(b"+").unwrap();
                }
                line_buf.write(b"[<").unwrap();
                for _ in 0..b {
                    if pos {
                        line_buf.write(b"+").unwrap();
                    } else {
                        line_buf.write(b"-").unwrap();
                    }
                }
                line_buf.write(b">-]<").unwrap();
            }
            for _ in 0..rem.abs() {
                if rem >= 0 {
                    line_buf.write(b"+").unwrap();
                } else {
                    line_buf.write(b"-").unwrap();
                }
            }
        }
        line_buf.write(b".").unwrap();
        cur = letter;

        // Now, outputting line buffer while inserting new lines as needed
        let line_pos = line_buf.position() as usize;
        let mut cur_pos = 0;
        line_buf.set_position(0);
        let buf = line_buf.get_ref();
        if line_len == 0 {
            // Just write whole line
            out.write(&buf[..line_pos]).unwrap();
        } else {
            // Printing as chunks
            while cur_pos != line_pos {
                // If there's not enough space on the line, just print that
                let printed = if line_pos - cur_pos >= line_len - last_written {
                    let to_print = line_len - last_written;
                    out.write(&buf[cur_pos..cur_pos + to_print]).unwrap();
                    out.write(b"\n").unwrap();
                    cur_pos += to_print;
                    to_print
                } else {
                    // Not enough space on the line, just printing what I can
                    let to_print = line_pos - cur_pos;
                    out.write(&buf[cur_pos..line_pos]).unwrap();
                    cur_pos = line_pos;
                    to_print
                };
                last_written = (last_written + printed) % line_len;
            }
        }
    }

    out.write(b"\n").unwrap();
    out.flush().unwrap();
}

// Wanted to use primes, but nested loops with brainfuck are actually very difficult apparently, so doing something simpler
// Any possible solution I can think of would be very unoptimized

// Returns 3 values. Two factors, and a remainder to add
fn fact_and_left(val: usize) -> (usize, usize, i64) {
    let root = (val as f64).sqrt();
    let a = root.round();
    let b = (val as f64 / a).round();
    let a = a as usize;
    let b = b as usize;
    let rem = val as i64 - (a * b) as i64;

    (a, b, rem)
}
