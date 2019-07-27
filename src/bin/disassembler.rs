use std::env;
use std::fs::File;
use std::io::prelude::*;


fn match_instruction(counter: i64, buffer: &mut [u8]){
    // TODO split this more so we can take into account every digit
    /* let opcode = (
        (buffer & 0xF000) >> 12 as u8,
        (buffer & 0x0F00) >> 8 as u8,
        (buffer & 0x00F0) >> 4 as u8,
        (buffer & 0x000F) as u8,
    ); */
    let opcode = (
        buffer[0],
        buffer[1],
    );
    match opcode {
        (0x00, 0xe0) => println!("{:x}: disp_clear()", counter),
        (0x00, 0xee) => println!("{:x}: return;", counter),
        _ => println!("Undefined Opcode on {:x}: {:x}{:x}", counter, opcode.0, opcode.1),
    }
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().into_iter().nth(1).unwrap();
    let mut file = File::open(file_name).unwrap();
    let mut buffer = [0; 2]; // 2 bytes
    let mut pc = 0;
    loop {
        match file.read_exact(&mut buffer) {
            Ok(_t) => {
                match_instruction(pc, &mut buffer);
                pc += 2;
            },
            Err(_err) => {
                // We hit this on EOF
                break Ok(());
            },
        }
    }
}
