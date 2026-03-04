pub fn combine_operands(operand1: u8, operand2: u8) -> u16 {
    return ((operand2 as u16) << 8) + (operand1 as u16);
}