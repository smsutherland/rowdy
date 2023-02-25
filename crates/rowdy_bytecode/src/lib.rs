use rowdy_macros::Bytes;

#[derive(Debug, Bytes)]
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

    /// Returns the length of the bytecode in bytes, *not* instructions.
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<'a> IntoIterator for &'a Bytecode {
    type Item = Instruction;

    type IntoIter = BytecodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BytecodeIter::new(self)
    }
}

pub struct BytecodeIter<'a> {
    bytecode: &'a Bytecode,
    index: usize,
}

impl<'a> BytecodeIter<'a> {
    fn new(bytecode: &'a Bytecode) -> Self {
        Self { bytecode, index: 0 }
    }
}

impl<'a> Iterator for BytecodeIter<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bytecode.len() {
            let (instruction, size) = Instruction::from_bytes(&self.bytecode.vec[self.index..]);
            self.index += size;
            Some(instruction)
        } else {
            None
        }
    }
}
