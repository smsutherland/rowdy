use rowdy_macros::AsBytes;

#[derive(Debug, AsBytes)]
pub enum Instruction {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Default)]
pub struct Bytecode {
    vec: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, instruction: Instruction) -> &mut Self {
        let (bytes, count) = instruction.as_bytes();
        self.vec.extend(&bytes[0..count]);
        self
    }
}
