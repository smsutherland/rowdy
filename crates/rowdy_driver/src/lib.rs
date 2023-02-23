use rowdy_compiler::{Compiler, Config};
use rowdy_type_checking::type_check;
use rowdy_lexer::tokenize;
use rowdy_parser::parse_tokens;

pub fn run(config: Config) {
    let compiler = Compiler::new(config).expect("TODO: handle errors here");
    let tokens = tokenize(&compiler);

    // for t in tokens.clone() {
    //     println!("{t:?}");
    // }

    let mut ast = parse_tokens(tokens, &compiler);
    // dbg!(&ast);

    type_check(&mut ast, &compiler);
}

#[test]
fn compile_testry() {
    let config = Config {
        source: Source::File("./test.ry".into()),
    };
    run(config);
}
