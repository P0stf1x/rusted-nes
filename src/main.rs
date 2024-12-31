use argparse::{ ArgumentParser, StoreFalse, StoreTrue, Store };

use std::num::Wrapping;

use crate::processor::*;
use crate::memory::*;

mod processor;
mod memory;

fn main() {
    let mut is_raw_image = false;
    let mut file_path = String::new();
    { // Limits argparse borrows to this scope
        let mut argparser = ArgumentParser::new();
        argparser.refer(&mut is_raw_image)
            .add_option(&["--ines"], StoreFalse, "Parse as iNES rom (Default)")
            .add_option(&["--raw"], StoreTrue, "Parse as raw image");
        argparser.refer(&mut file_path)
            .add_argument("rom image", Store, "Path to rom image");
        argparser.parse_args_or_exit();
    }
    let mut memory;
    if is_raw_image {
        memory = MEM::new_from(&file_path);
    } else {
        memory = MEM::new_from_ines(&file_path);
    }
    let mut cpu: CPU = Default::default();

    // TODO: move to cpu init
    cpu.reset(&mut memory);
    cpu.S = Wrapping(0xFDu8);
    cpu.I = true;
    memory.data[0x2002] = 0b_1000_0000; // FIXME: hack to make cpu think it's always in vblank
    
    use std::io::Write;
    use std::fs;
    let file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        // either use the ? operator or unwrap since it returns a Result
        .open("initial_memory.dump");

    let _ = match file {
        Ok(mut f) => f.write_all(&memory.data),
        Err(_) => Ok(println!("No file")),
    };
    
    loop {
        if cpu.execute(&mut memory).is_err() {
            // TODO: use logger instead
            println!("");
            println!("-----------------------------");
            println!("WE CRASHED");
            println!("{:#04X?}", cpu);
            println!("{:#04X}", memory.read(cpu.PC.0 as usize, 1));
            println!("-----------------------------");
            panic!();
        };
    }
}
