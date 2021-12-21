use crate::token::{self, Token, DataType};
use std::collections::HashMap;

pub fn compile_tokens(tokens: &Vec<Token>) {
    // let entry = find_entry(&tokens).unwrap();
    // println!("starting at token {}", entry);

    let mut functions: HashMap<FunctionSignature, Vec<Token>> = HashMap::new();

    let mut i = 0;
    while i < tokens.len(){
        if let token::TokenType::TokenFuncDecl(token::FunctionDecl{return_type, name}) = &tokens[i].typ{
            i += 1;
            debug_assert!(matches!(tokens[i].typ, token::TokenType::TokenSpecialChar(token::SpecialChar::LParen(_))));
            let mut func_def = FunctionSignature{
                return_type: *return_type,
                name: String::from(name),
                parameters: Vec::new(),
            };
            i += 1;

            while let token::TokenType::TokenVarDecl(token::VarDecl{var_type, name}) = &tokens[i].typ{
                func_def.parameters.push((*var_type, String::from(name)));
                i += 1;
            }
            debug_assert!(matches!(tokens[i].typ, token::TokenType::TokenSpecialChar(token::SpecialChar::RParen(_))));
            i += 1;
            if let token::TokenType::TokenSpecialChar(token::SpecialChar::LBrace(Some(end_index))) = tokens[i].typ{
                i += 1;
                functions.insert(func_def, tokens[i..end_index].to_vec());
                i = end_index+1;
            }
            else{
                unreachable!();
            }
        }
        else{
            i += 1;
        }
    }

    println!("{:#?}", functions);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct FunctionSignature{
    return_type: DataType,
    name: String,
    parameters: Vec<(DataType, String)>,
}

// fn find_entry(tokens: &Vec<Token>) -> Option<usize>{
//     let mut entry_index = None;
//     for (i, token) in tokens.iter().enumerate() {
//         if token.is_entry(){
//             if entry_index == None{
//                 entry_index = Some(i);
//             }
//             else{
//                 panic!("multiple entry points");
//             }
//         }
//     }
//     entry_index
// }
// 0: Token { typ: TokenFuncDecl(FunctionDecl { return_type: Int, name: "main" }), loc: .\test.ry:1:1 }