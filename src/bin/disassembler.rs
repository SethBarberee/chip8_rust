use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_name = env::args().into_iter().nth(1).unwrap();
    let file = File::open(file_name).unwrap();
    let mut pc = 0;
    for byte in file.bytes() {
        println!("{:x}: {:x}", pc, byte.unwrap());
        // TODO need the next byte
        // TODO do a match here on the bytes
        pc += 1;
    }
    Ok(())
}
