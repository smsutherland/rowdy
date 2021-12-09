use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub loc: Location,
}

impl Token {
    pub fn new(typ: TokenType, loc: Location) -> Token {
        Token { typ, loc }
    }
}

#[derive(Clone)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum TokenType {
    TokenSymbol(String),
    TokenOperator(Operator),
    TokenDataType(DataType),
    TokenSpecialChar(SpecialChar),
    TokenFuncDecl(FunctionDecl),
    TokenVarDecl(VarDecl),
    TokenFuncCall(String),
}

impl TokenType {
    pub fn new_func_decl(return_type: DataType, name: String) -> TokenType {
        TokenType::TokenFuncDecl(FunctionDecl { return_type, name })
    }

    pub fn new_var_decl(var_type: DataType, name: String) -> TokenType {
        TokenType::TokenVarDecl(VarDecl { var_type, name })
    }

    pub fn from_string(token_str: &str) -> Self {
        use DataType::*;
        use Operator::*;
        use SpecialChar::*;
        use TokenType::*;

        match token_str {
            "+" => TokenOperator(Plus),
            "-" => TokenOperator(Sub),
            "=" => TokenOperator(Assign),
            "==" => TokenOperator(Equal),
            ";" => TokenOperator(End),

            "int" => TokenDataType(Int),

            "(" => TokenSpecialChar(LParen(None)),
            ")" => TokenSpecialChar(RParen(None)),
            "{" => TokenSpecialChar(LBrace(None)),
            "}" => TokenSpecialChar(RBrace(None)),
            "[" => TokenSpecialChar(LBracket(None)),
            "]" => TokenSpecialChar(RBracket(None)),

            _ => TokenSymbol(String::from(token_str)),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenSymbol(_) => write!(f, "symbol"),
            Self::TokenOperator(_) => write!(f, "operator"),
            Self::TokenDataType(_) => write!(f, "data type"),
            Self::TokenSpecialChar(_) => write!(f, "special char"),
            Self::TokenFuncDecl(_) => write!(f, "function declaration"),
            Self::TokenVarDecl(_) => write!(f, "variable declaration"),
            Self::TokenFuncCall(_) => write!(f, "function call"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Int,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Sub,
    Assign,
    Equal,
    End,
}

#[derive(Debug)]
pub enum SpecialChar {
    LParen(Option<usize>),
    RParen(Option<usize>),
    LBrace(Option<usize>),
    RBrace(Option<usize>),
    LBracket(Option<usize>),
    RBracket(Option<usize>),
}

#[derive(Debug)]
pub struct FunctionDecl {
    return_type: DataType,
    name: String,
}

#[derive(Debug)]
pub struct VarDecl {
    var_type: DataType,
    name: String,
}
