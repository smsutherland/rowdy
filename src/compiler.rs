use crate::token::{self, DataType, Token};
use std::collections::HashMap;

type FunctionMap = HashMap<FunctionSignature, Vec<Token>>;
pub fn compile_tokens(tokens: &Vec<Token>) {
    let functions = make_functions(tokens);

    let main_signature: FunctionSignature = FunctionSignature {
        return_type: DataType::Int,
        name: String::from("main"),
        parameters: Vec::new(),
    };

    assert!(
        functions.contains_key(&main_signature),
        "program needs main function"
    );

    println!("{:#?}", functions);
}

fn make_functions(tokens: &Vec<Token>) -> FunctionMap {
    let mut functions: FunctionMap = HashMap::new();
    let mut i = 0;
    while i < tokens.len() {
        if let token::TokenType::TokenFuncDecl(token::FunctionDecl { return_type, name }) =
            &tokens[i].typ
        {
            i += 1;
            debug_assert!(matches!(
                tokens[i].typ,
                token::TokenType::TokenSpecialChar(token::SpecialChar::LParen(_))
            ));
            let mut func_def = FunctionSignature {
                return_type: *return_type,
                name: String::from(name),
                parameters: Vec::new(),
            };
            i += 1;

            while let token::TokenType::TokenVarDecl(token::VarDecl { var_type, name }) =
                &tokens[i].typ
            {
                func_def.parameters.push((*var_type, String::from(name)));
                i += 1;
            }

            debug_assert!(matches!(
                tokens[i].typ,
                token::TokenType::TokenSpecialChar(token::SpecialChar::RParen(_))
            ));

            assert!(check_for_duplicate_function(functions.keys(), &func_def));

            i += 1;
            if let token::TokenType::TokenSpecialChar(token::SpecialChar::LBrace(Some(end_index))) =
                tokens[i].typ
            {
                i += 1;
                functions.insert(func_def, tokens[i..end_index].to_vec());
                i = end_index + 1;
            } else {
                unreachable!();
            }
        } else {
            i += 1;
        }
    }
    functions
}

fn check_for_duplicate_function<'a>(
    existing_signatures: impl Iterator<Item = &'a FunctionSignature>,
    new_signature: &FunctionSignature,
) -> bool {
    for signature in existing_signatures {
        if !new_signature.is_compatible(&signature) {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct FunctionSignature {
    return_type: DataType,
    name: String,
    parameters: Vec<(DataType, String)>,
}

impl FunctionSignature {
    fn is_compatible(&self, other: &Self) -> bool {
        self.name != other.name
            || !self
                .parameters
                .iter()
                .map(|x| x.0)
                .eq(other.parameters.iter().map(|x| x.0))
    }
}
