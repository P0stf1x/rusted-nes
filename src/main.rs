use std::num::Wrapping;

use crate::processor::*;
use crate::memory::*;

mod processor;
mod memory;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut memory;
    if args.len() > 1 {
        memory = MEM::new_from(&args[1]);
    } else {
        memory = MEM::new(MEMORY_SIZE);
    }
    let mut cpu: CPU = CPU::new();

    // TODO: move to cpu init
    cpu.PC = std::num::Wrapping(PRG_ROM_ENTRY_ADDR as u16);
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
