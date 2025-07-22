pub enum Opcode{
    PUSH,
    ADD, // ignore the operand
    MUL, // ignore the operand
    NOP, // ignore the operand
}

pub struct Instruction{
    pub opcode: Opcode,
    pub operand: u8,
}

impl Instruction{
    pub fn add(num: u8) -> Instruction{
        Instruction {
            opcode: Opcode::ADD,
            operand: num,
        }
    }
}