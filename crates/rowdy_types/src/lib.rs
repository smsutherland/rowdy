pub type TypeID = u32;

#[derive(Debug)]
pub struct FnSignature {
    #[allow(dead_code)]
    return_type: TypeID,
    #[allow(dead_code)]
    parameter_types: Vec<TypeID>,
}
