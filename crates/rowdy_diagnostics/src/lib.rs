use rowdy_location::Span;
use rowdy_compiler::Compiler;

struct Diagnostic<'a> {
    span: Span,
    relavent_str: &'a str,
    error_kind: ErrorKind,
    level: Level,
    compiler: &'a Compiler,
}

impl Diagnostic<'_> {
    fn get_pad(&self) -> usize {
        let last_line = self.span.end.line;
        let mut padding = 0;
        let mut line_counter = last_line;
        while line_counter > 0 {
            line_counter /= 10;
            padding += 1;
        }
        (padding + 1).max(4)
    }

    fn pad_after_line_num(&self) -> usize {
        let last_line = self.span.end.line;
        let mut padding = 0;
        let mut line_counter = last_line;
        while line_counter > 0 {
            line_counter /= 10;
            padding += 1;
        }
        self.get_pad() - padding
    }

    fn write_padding(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left_padding = self.get_pad();
        f.write_str(&" ".repeat(left_padding))
    }
}

impl std::fmt::Display for Diagnostic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}[{:?}]: {}",
            self.level, self.error_kind, self.error_kind
        )?;
        let pad = self.get_pad();
        f.write_str(&" ".repeat(pad - 1))?;
        writeln!(f, "--> {}:{}", self.compiler.config.source, self.span.start)?;
        write!(
            f,
            "{}{}| {}",
            self.span.start.line,
            " ".repeat(self.pad_after_line_num()),
            self.relavent_str
        )?;

        Ok(())
    }
}

#[derive(Debug)]
enum Level {
    Error,
    Warning,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Error => f.write_str("error"),
            Level::Warning => f.write_str("warning"),
        }
    }
}

pub fn print_error(span: Span, message: ErrorKind, compiler: &Compiler) {
    let diagnostic = Diagnostic {
        span,
        error_kind: message,
        relavent_str: span.slice(&compiler.code),
        level: Level::Error,
        compiler,
    };

    eprintln!("{diagnostic}");
}

#[derive(Debug)]
pub enum ErrorKind {
    E0000,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::E0000 => f.write_str("unknown error"),
        }
    }
}
