use std::env;
use std::io;
use text_to_brainfuck::*;

// Initial version
// Not much optimization, just some looping to adjust a single value as needed

const DEFAULT_LEN: usize = 24;

fn main() {
    let mut args = env::args();
    // Discarding first arg
    args.next();
    // Then parsing second if it exists
    let line_len: Result<usize, &str> = match args.next() {
        Some(val) => {
            // If the string can be parsed, return that, otherwise error
            match val.parse::<usize>() {
                Ok(num) => Ok(num),
                Err(_) => Err("Error: Unable to parse line length"),
            }
        }
        None => Ok(DEFAULT_LEN),
    };
    match line_len {
        Ok(num) => parse(io::stdin(), io::stdout(), num),
        Err(err) => eprintln!("{}", err),
    };
}
