mod expression;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub use expression::{BinaryOp, Expression};

mod function;
pub use function::{Function, Pattern};

mod literal;
pub use literal::Literal;

mod list;
pub use list::List;

pub type ParseStream<'a> = Peekable<Enumerate<Chars<'a>>>;

pub fn parse_stream(s: &str) -> ParseStream<'_> {
    s.chars().enumerate().peekable()
}

fn parse_char(chars: &mut ParseStream, c: char) -> Result<usize, Error> {
    if let Some((pos, next_c)) = chars.next() {
        if next_c == c {
            Ok(pos)
        } else {
            Err(Error::new(Span::new(pos, pos)))
        }
    } else {
        Err(Error::empty())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    span: Option<Span>,
}

impl Error {
    pub fn new(span: Span) -> Self {
        Self { span: Some(span) }
    }

    pub fn empty() -> Self {
        Self { span: None }
    }
}
