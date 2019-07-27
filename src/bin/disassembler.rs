use std::env;
use std::fs::File;
use std::io::prelude::*;


fn match_instruction(counter: i64, buffer: &Vec<u8>){
    // Get opcode from the two bytes
    let opcode = (
        (buffer[0] & 0xF0) >> 4 as u8,
        (buffer[0] & 0x0F) as u8,
        (buffer[1] & 0xF0) >> 4 as u8,
        (buffer[1] & 0x0F) as u8,
    );
    match opcode {
        (0x0, 0x0, 0xe, 0x0) => println!("Line {:x}: disp_clear()", counter),
        (0x0, 0x0, 0xe, 0xe) => println!("Line {:x}: return;", counter),
        (0x0, _, _, _) => println!("Line {:x}: RCA Program called", counter), // haven't seen this one yet
        (0x1, _, _, _) => println!("Line {:x}: goto {:x}{:x}{:x}", counter, opcode.1, opcode.2, opcode.3),
        (0x2, _, _, _) => println!("Line {:x}: *(0x{:x}{:x})()", counter, opcode.2, opcode.3),
        (0x3, _, _, _) => println!("Line {:x}: if Register {:x} == {:x}{:x} (Skip next if true)", counter, opcode.1, opcode.2, opcode.3),
        (0x4, _, _, _) => println!("Line {:x}: if Register {:x} != {:x}{:x} (Skip next if not true)", counter, opcode.1, opcode.2, opcode.3),
        (0x5, _, _, _) => println!("Line {:x}: if Register {:x} == Register {:x} (Skip next if true)", counter, opcode.1, opcode.2),
        (0x6, _, _, _) => println!("Line {:x}: Register {:x} = {:x}{:x} (Const)", counter, opcode.1, opcode.2, opcode.3),
        (0x7, _, _, _) => println!("Line {:x}: Register {:x} += {:x}{:x} (carry not set)", counter, opcode.1, opcode.2, opcode.3),
        (0x8, _, _, 0x0) => println!("Line {:x}: Register {:x} = {:x} (Assign)", counter, opcode.1, opcode.2),
        (0xa, _, _, _) => println!("Line {:x}: I = {:x}{:x}{:x}", counter, opcode.1, opcode.2, opcode.3),
        (0xd, _, _, _) => println!("Line {:x}: draw at ({:x},{:x}) with height {:x} pixels", counter, opcode.1, opcode.2, opcode.3),
        (0xe, _, _, _) => println!("Line {:x}: Key check", counter),
        (0xf, _, 0x0, 0x7) => println!("Line {:x}: Register {:x} = delay_timer", counter, opcode.1),
        (0xf, _, 0x1, 0x5) => println!("Line {:x}: Delay timer = {:x}", counter, opcode.1),
        (0xf, _, 0x1, 0x8) => println!("Line {:x}: Sound timer = {:x}", counter, opcode.1),
        (0xf, _, 0x2, 0x9) => println!("Line {:x}: I = Address of sprite in Register {:x}", counter, opcode.1),
        _ => println!("Undefined Opcode on Line {:x}: {:x}{:x}{:x}{:x}", counter, opcode.0, opcode.1, opcode.2, opcode.3),
    }
}

fn main() -> std::io::Result<()> {
    let file_name = env::args().into_iter().nth(1).unwrap();
    let mut file = File::open(file_name).unwrap();
    let mut buffer = vec![0; 2];

    let mut pc = 0;
    loop {
        match file.read_exact(&mut buffer) {
            Ok(_t) => {
                // We borrow here size we move again in the loop
                match_instruction(pc, &buffer);
                pc += 2;
            },
            Err(_err) => {
                // We hit this on EOF
                break Ok(());
            },
        }
    }
}
