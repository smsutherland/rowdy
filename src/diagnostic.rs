use crate::location::Span;
use crate::Compiler;

struct Diagnostic<'a> {
    span: Span,
    relavent_str: &'a str,
    message: String,
    level: Level,
}

impl std::fmt::Display for Diagnostic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} - {}: {}",
            self.span.start.line, self.span.start.col, self.relavent_str, self.message
        )
    }
}

enum Level {
    Error,
    Warning,
}

pub fn print_error(span: Span, message: String, compiler: &Compiler) {
    let diagnostic = Diagnostic {
        span,
        message,
        relavent_str: span.slice(&compiler.code),
        level: Level::Error,
    };

    eprintln!("{diagnostic}");
}
