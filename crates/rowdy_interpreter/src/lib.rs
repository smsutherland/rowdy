use rowdy_bytecode::{Bytecode, Instruction};

pub fn interpret_bytecode(bytecode: Bytecode) -> i32 {
    for instruction in &bytecode {
        interpret(instruction)
    }
    0
}

fn interpret(instruction: Instruction) {
    dbg!(instruction);
}
