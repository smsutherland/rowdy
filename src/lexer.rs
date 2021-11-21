use std::fmt;

#[derive(Debug)]
enum TokenType{
    TokenString(String),
}

#[derive(Debug)]
struct Location{
    file: String,
    line: usize,
    col: usize,
}

impl std::string::ToString for Location{
    fn to_string(&self) -> String{
        String::from(format!("{}:{}:{}", self.file, self.line, self.col))
    }
}

#[derive(Debug)]
pub struct Token{
    typ: TokenType,
    loc: Location,
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
    Ok(tokens)
}

fn lex_line(line: &str, fname: &str, line_number: usize) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut i: usize = 1;
    for token in line.split(' '){
        tokens.push(Token{
            typ: TokenType::TokenString(String::from(token)),
            loc: Location{
                file: String::from(fname),
                line: line_number,
                col: i,
            }
        });
        i += token.len() + 1;
    }
    tokens
}