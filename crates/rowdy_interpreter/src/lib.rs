use rowdy_bytecode::{Bytecode, Instruction};

pub fn interpret_bytecode(bytecode: Bytecode) -> i32 {
    let mut interpreter = Interpreter::new(bytecode);
    interpreter.run()
}

struct Interpreter {
    bytecode: Bytecode,
    stack: Vec<i32>,
}

impl Interpreter {
    fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            stack: Vec::new(),
        }
    }

    fn run(&mut self) -> i32 {

macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr {
            Some(a) => a,
            None => return -1,
        }
    };

    ($expr:expr, $ret:expr) => {
        match $expr {
            Some(a) => a,
            None => return $ret,
        }
    };
}

        for instruction in self.bytecode.iter() {
            match instruction {
                Instruction::Push(num) => self.stack.push(num),
                Instruction::Pop => {
                    unwrap_or_return!(self.stack.pop());
                }
                Instruction::Add => {
                    let a = unwrap_or_return!(self.stack.pop());
                    let b = unwrap_or_return!(self.stack.pop());
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let a = unwrap_or_return!(self.stack.pop());
                    let b = unwrap_or_return!(self.stack.pop());
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let a = unwrap_or_return!(self.stack.pop());
                    let b = unwrap_or_return!(self.stack.pop());
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let a = unwrap_or_return!(self.stack.pop());
                    let b = unwrap_or_return!(self.stack.pop());
                    self.stack.push(a / b);
                }
            }
        }
        0
    }
}
