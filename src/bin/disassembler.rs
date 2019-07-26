use std::env;
use std::fs::File;
use std::io::prelude::*;


fn match_instruction(counter: i64, opcode: &mut [u8]){
    // TODO buffer holds opcode so match on the two values
    match opcode {
        _ => println!("Undefined Opcode on {}: {:x}{:x}", counter, opcode[0], opcode[1]),
    }
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().into_iter().nth(1).unwrap();
    let mut file = File::open(file_name).unwrap();
    let mut buffer = [0; 2]; // 2 bytes
    let mut pc = 0;
    loop {
        match file.read_exact(&mut buffer) {
            Ok(T) => {
                match_instruction(pc, &mut buffer);
                pc += 2;
            },
            Err(err) => {
                // We hit this on EOF
                break Ok(());
            },
        }
    }
}
