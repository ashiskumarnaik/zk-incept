use crate::incept::Incept;
use crate::instructions::Instruction;

mod incept;
mod instructions;

fn main() {
    let mut vm:Incept = Incept::new();
    // println!("{:?}", vm);

    // for a series of instruction, need to creat a vetor

    let mut instructions: Vec<instructions::Instruction> = Vec::new();
    instructions.push(Instruction::add(3))
}

