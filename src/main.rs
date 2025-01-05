use argparse::{ ArgumentParser, StoreFalse, StoreTrue, Store, ParseOption };

use std::num::Wrapping;
use std::thread;

use crate::processor::*;
use crate::memory::*;
use crate::pixel_processor::*;

mod processor;
mod memory;
mod pixel_processor;

fn main() {
    let mut is_raw_image = false;
    let mut entry_point: Option<usize> = None;
    let mut file_path = String::new();
    { // Limits argparse borrows to this scope
        let mut argparser = ArgumentParser::new();
        argparser.refer(&mut is_raw_image)
            .add_option(&["--ines"], StoreFalse, "Parse as iNES rom (Default)")
            .add_option(&["--raw"], StoreTrue, "Parse as raw image");
        argparser.refer(&mut entry_point)
            .add_option(&["-e", "--entry-point"], ParseOption, "Manually choose cpu entry point"); // HEX not supported
        argparser.refer(&mut file_path)
            .add_argument("rom image", Store, "Path to rom image").required();
        argparser.parse_args_or_exit();
    }
    let mut memory;
    let mut ppu_memory;
    if is_raw_image {
        memory = MEM::new_from(&file_path);
        unimplemented!();
    } else {
        (memory, ppu_memory) = MEM::new_from_ines(&file_path);
    }
    let mut cpu: CPU = CPU::new();

    // TODO: move to cpu init
    match entry_point {
        None => cpu.reset(&mut memory),
        Some(address) => cpu.PC = Wrapping(address as u16)
    }
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

    // Memory pointer since in rust pointers aren't Sync/Send
    let memory_pointer = MemPtrWrapper(&mut memory as *mut MEM);

    let thread_handle = thread::spawn(move || { // macOS doesn't like it when window is created not from main thread so we put cpu on the other thread 🤷🏻‍♂️
        while cpu.execute(&mut memory).is_ok() { // emulator loop
        }
        // TODO: use logger instead
        println!("");
        println!("-----------------------------");
        println!("WE CRASHED");
        println!("{:#04X?}", cpu);
        println!("{:#04X}", memory.read(cpu.PC.0 as usize, 1));
        println!("-----------------------------");
    });

    let mut ppu = PPU::new(memory_pointer, ppu_memory);
    ppu.run();
}
