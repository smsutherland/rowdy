use crate::location::Span;
use crate::Compiler;

struct Diagnostic {
    span: Span,
    message: String,
    level: Level,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

enum Level {
    Error,
    Warning,
}

pub fn print_error(span: Span, message: String, _compiler: &Compiler) {
    let diagnostic = Diagnostic {
        span,
        message,
        level: Level::Error,
    };

    println!("{diagnostic}");
}
