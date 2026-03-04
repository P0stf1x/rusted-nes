use crate::CPU;

pub struct Logger;

impl Logger {
    pub fn log_cpu_instruction(cpu: &CPU, instruction: u8, operand1: Option<u8>, operand2: Option<u8>, decoded_instruction: String) {
        // TODO: use proper logger
        print!("{:04X}  ", cpu.get_pc());
        print!("{instruction:02X} ");
        match operand1 {
            Some(operand) => print!("{operand:02X} "),
            None => print!("   "),
        }
        match operand2 {
            Some(operand) => print!("{operand:02X}  "),
            None => print!("    "),
        }
        print!("{decoded_instruction: <32}");
        print!("A:{:02X} ", cpu.get_a());
        print!("X:{:02X} ", cpu.get_x());
        print!("Y:{:02X} ", cpu.get_y());
        print!("P:{:02X} ", cpu.store_status());
        print!("SP:{:02X} ", cpu.S.0);
        println!();
    }
}
