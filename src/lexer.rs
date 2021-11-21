use std::fmt;

#[derive(Debug)]
pub struct Token{
    typ: TokenType,
    loc: Location,
}

#[derive(Debug, Clone)]
struct Location{
    file: String,
    line: usize,
    col: usize,
}

impl fmt::Display for Location{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

#[derive(Debug)]
enum TokenType{
    TokenString(String),
    TokenOperator(Operator),
    TokenDataType(DataType),
    TokenSpecialChar(SpecialChar),
    TokenFunctionDecl(FunctionDecl),
    TokenVarDecl(VarDecl),
}

impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::TokenString(_) => write!(f, "string"),
            Self::TokenOperator(_) => write!(f, "operator"),
            Self::TokenDataType(_) => write!(f, "data type"),
            Self::TokenSpecialChar(_) => write!(f, "special char"),
            Self::TokenFunctionDecl(_) => write!(f, "function declaration"),
            Self::TokenVarDecl(_) => write!(f, "variable declaration"),
        }
    }
}

impl TokenType{
    fn new(token_str: String) -> Self{
        use TokenType::*;
        use Operator::*;
        use DataType::*;
        use SpecialChar::*;

        match token_str.as_str(){
            "+" => TokenOperator(Plus),
            "-" => TokenOperator(Sub),
            "=" => TokenOperator(Assign),
            "==" => TokenOperator(Equal),
            ";" => TokenOperator(End),

            "int" => TokenDataType(Int),

            "(" => TokenSpecialChar(LParen),
            ")" => TokenSpecialChar(RParen),
            "{" => TokenSpecialChar(LBrace),
            "}" => TokenSpecialChar(RBrace),
            "[" => TokenSpecialChar(LBracket),
            "]" => TokenSpecialChar(RBracket),

            _ => TokenString(token_str),
        }
    }
}

#[derive(Debug, Clone)]
enum DataType{
    Int,
}

#[derive(Debug)]
enum Operator{
    Plus,
    Sub,
    Assign,
    Equal,
    End,
}

#[derive(Debug)]
enum SpecialChar{
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

#[derive(Debug)]
struct FunctionDecl{
    return_type: DataType,
    name: String,
}

#[derive(Debug)]
struct VarDecl{
    var_type: DataType,
    name: String,
}

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
            tokens.push(Token{
                typ: TokenType::new(String::from(token)),
                loc: Location{
                    file: String::from(fname),
                    line: line_number,
                    col: i,
                }
            });
        }
        i += token.len() + 1;
    }
    tokens
}

const MAX_MATCH_LEN: usize = 3;


fn make_multi_token_objects(mut tokens: Vec<Token>) -> Result<Vec<Token>, String>{
    let mut tokens: Vec<Token> = tokens.drain(..).rev().collect();
    let mut result: Vec<Token> = Vec::new();

    while tokens.len() != 0 {
        let slice;
        if  tokens.len() < MAX_MATCH_LEN {
            slice = &tokens[..];
        }
        else{
            slice = &tokens[tokens.len()-MAX_MATCH_LEN..];
        }
        if let Some((new_token, len)) = matches_function_decl(slice){
            result.push(new_token);
            for _ in 0..len{
                tokens.pop();
            }
            continue;
        }
        if let Some((new_token, len)) = matches_var_decl(slice){
            result.push(new_token);
            for _ in 0..len{
                tokens.pop();
            }
            continue;
        }

        result.push(tokens.pop().unwrap());
    }

    Ok(result)
}

fn matches_function_decl(tokens: &[Token]) -> Option<(Token, usize)> {
    if tokens.len() < 3{
        return None;
    }

    use TokenType::*;
    use SpecialChar::*;

    if matches!(&tokens[2].typ, TokenDataType(_)){
    if matches!(&tokens[1].typ, TokenString(_)){
    if matches!(&tokens[0].typ, TokenSpecialChar(c) if matches!(c, LParen)){
        let return_type = match &tokens[2].typ{
            TokenDataType(typ) => typ.clone(),
            _ => unreachable!()
        };

        let name = match &tokens[1].typ{
            TokenString(name) => name.clone(),
            _ => unreachable!()
        };

        return Some(
            (
                Token{
                    typ: TokenFunctionDecl(
                        FunctionDecl{
                            return_type,
                            name,
                        }
                    ),
                    loc: tokens[2].loc.clone()
                },
                2
            )
        );
    }}}

    None
}

fn matches_var_decl(tokens: &[Token]) -> Option<(Token, usize)> {
    if tokens.len() < 2{
        return None;
    }

    use TokenType::*;

    if matches!(&tokens[1].typ, TokenDataType(_)){
    if matches!(&tokens[0].typ, TokenString(_)){
        let var_type = match &tokens[1].typ{
            TokenDataType(typ) => typ.clone(),
            _ => unreachable!()
        };

        let name = match &tokens[0].typ{
            TokenString(name) => name.clone(),
            _ => unreachable!()
        };
        return Some(
            (
                Token{
                    typ: TokenVarDecl(
                        VarDecl{
                            var_type,
                            name,
                        }
                    ),
                    loc: tokens[2].loc.clone()
                },
                2
            )
        );
    }}

    None
}