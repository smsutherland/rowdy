use super::token::*;

pub fn lex_file(filename: &String) -> Result<Vec<Token>, String>{
    let lines = match super::read_lines(&filename){
        Ok(val) => val,
        Err(_) => return Err(String::from(format!("Could not read file {}", &filename)))
    };

    let mut tokens = Vec::new();

    let mut i = 0;
    for line in lines{
        i += 1;
        if let Ok(line) = line{
            let mut line_tokens = lex_line(&line, filename, i);
            tokens.append(&mut line_tokens);
        }
    }

    Ok(make_multi_token_objects(tokens)?)
}

fn lex_line(line: &str, fname: &str, line_number: usize) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut i: usize = 1;
    for token in line.split(' '){
        if token.len() > 0{
            tokens.push(Token::new(TokenType::from_string(token), Location{
                file: String::from(fname),
                line: line_number,
                col: i,
            }));
        }
        i += token.len() + 1;
    }
    tokens
}

fn make_multi_token_objects(mut tokens: Vec<Token>) -> Result<Vec<Token>, String>{
    let mut rtokens: Vec<Token> = tokens.drain(..).rev().collect();
    let mut result: Vec<Token> = Vec::new();

    while rtokens.len() != 0 {
        // println!("{:#?}", rtokens);
        if let Some((new_token, len)) = matches_function_decl(&rtokens){
            result.push(new_token);
            for _ in 0..len{
                rtokens.pop();
            }
            continue;
        }
        if let Some((new_token, len)) = matches_var_decl(&rtokens){
            result.push(new_token);
            for _ in 0..len{
                rtokens.pop();
            }
            continue;
        }
        if let Some((new_token, len)) = matches_func_call(&rtokens){
            result.push(new_token);
            for _ in 0..len{
                rtokens.pop();
            }
            continue;
        }

        result.push(rtokens.pop().unwrap());
    }

    Ok(result)
}

fn matches_function_decl(mut tokens: &[Token]) -> Option<(Token, usize)> {
    let pattern_len = 3;
    if tokens.len() < pattern_len{
        return None;
    }
    else{
        tokens = &tokens[tokens.len()-pattern_len..];
    }

    use TokenType::*;
    use SpecialChar::*;

    if matches!(&tokens[2].typ(), TokenDataType(_)){
    if matches!(&tokens[1].typ(), TokenSymbol(_)){
    if matches!(&tokens[0].typ(), TokenSpecialChar(c) if matches!(c, LParen)){
        let return_type = match &tokens[2].typ(){
            TokenDataType(typ) => typ.clone(),
            _ => unreachable!()
        };

        let name = match &tokens[1].typ(){
            TokenSymbol(name) => name.clone(),
            _ => unreachable!()
        };

        return Some(
            (
                Token::new(
                    TokenType::new_func_decl(return_type, name),
                    tokens[2].loc().clone()
                ),
                2
            )
        );
    }}}

    None
}

fn matches_var_decl(mut tokens: &[Token]) -> Option<(Token, usize)> {
    let pattern_len = 2;
    if tokens.len() < pattern_len{
        return None;
    }
    else{
        tokens = &tokens[tokens.len()-pattern_len..];
    }

    use TokenType::*;

    if matches!(&tokens[1].typ(), TokenDataType(_)){
    if matches!(&tokens[0].typ(), TokenSymbol(_)){
        let var_type = match &tokens[1].typ(){
            TokenDataType(typ) => typ.clone(),
            _ => unreachable!()
        };

        let name = match &tokens[0].typ(){
            TokenSymbol(name) => name.clone(),
            _ => unreachable!()
        };
        return Some(
            (
                Token::new(
                    TokenType::new_var_decl(var_type, name),
                    tokens[1].loc().clone()
                ),
                2
            )
        );
    }}

    None
}

fn matches_func_call(mut tokens: &[Token]) -> Option<(Token, usize)> {
    let pattern_len = 2;
    if tokens.len() < pattern_len{
        return None;
    }
    else{
        tokens = &tokens[tokens.len()-pattern_len..];
    }

    use TokenType::*;
    use SpecialChar::*;

    if matches!(&tokens[1].typ(), TokenSymbol(_)){
    if matches!(&tokens[0].typ(), TokenSpecialChar(c) if matches!(c, LParen)){
        let name = match &tokens[1].typ(){
            TokenSymbol(name) => name.clone(),
            _ => unreachable!()
        };
        return Some(
            (
                Token::new(
                    TokenType::TokenFuncCall(name),
                    tokens[1].loc().clone()
                ),
                1
            )
        );
    }}

    None
}
