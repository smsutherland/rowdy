pub type TypeID = u32;

#[derive(Debug)]
pub struct FnSignature {
    pub return_type: TypeID,
    pub parameter_types: Vec<TypeID>,
}
