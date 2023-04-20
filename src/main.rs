use std::fs::File;
use std::io::{prelude::*, BufReader};
mod disassembler;

fn main() {
    // Read file as buffer and convert buffer to byte vector
    let mut f_read = get_buf_reader("space-invaders/invaders.h");
    let mut buffer = Vec::new();
    if let Err(e) = f_read.read_to_end(&mut buffer) {
        println!("Could not read buffer, Error: {}", e.to_string());
        return;
    }

    // Loop through byte vector
    let f_size = buffer.len();
    let mut pc = 0;
    while pc < f_size {
        pc += disassembler::disassemble8080p(&buffer, pc);
    }
}


fn get_buf_reader(filename: &str) -> BufReader<File> {
    let f = File::open(filename)
        .expect("failed to open file");
    return BufReader::new(f);
}

