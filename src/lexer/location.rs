use std::fmt;

#[derive(Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub char_num: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Source<'a> {
    File(&'a str),
    Anonymous,
}

#[derive(Clone, Copy)]
pub struct SourceLocation<'a> {
    pub file: Source<'a>,
    pub loc: Location,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    // pub file: Source<'a>,
    pub start: Location,
    pub end: Location,
}

impl Location {
    fn add_source(self, source: Source) -> SourceLocation {
        SourceLocation {
            file: source,
            loc: self,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for SourceLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            match self.file {
                Source::File(file) => file,
                Source::Anonymous => "anon",
            },
            self.loc.line,
            self.loc.col
        )
    }
}

impl fmt::Debug for SourceLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Span {
    pub fn from_loc(loc: Location) -> Self {
        Self {
            // file: Source::Anonymous,
            start: loc,
            end: loc,
        }
    }

    pub fn from_source_loc<'a>(loc: SourceLocation<'a>) -> Self {
        Self {
            // file: loc.file,
            start: loc.loc,
            end: loc.loc,
        }
    }

    pub fn from_start_end(start: Location, end: Location) -> Self {
        Self {
            // file: Source::Anonymous,
            start,
            end,
        }
    }

    pub fn from_source_start_end<'a>(start: SourceLocation<'a>, end: SourceLocation<'a>) -> Self {
        if start.file != end.file {
            panic!("Cannot create a span across multiple files.");
        }
        Self {
            // file: start.file,
            start: start.loc,
            end: end.loc,
        }
    }

    pub fn slice<'b>(&self, s: &'b str) -> &'b str {
        &s[self.start.char_num..=self.end.char_num]
    }
}
