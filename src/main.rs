use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub type ParseStream<'a> = Peekable<Enumerate<Chars<'a>>>;

pub fn parse_stream(s: &str) -> ParseStream<'_> {
    s.chars().enumerate().peekable()
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Literal {
    Int { value: String, span: Span },
}

impl Literal {
    pub fn parse(chars: &mut ParseStream) -> Self {
        let (start, first) = chars
            .peek()
            .filter(|(_, c)| c.is_ascii_digit())
            .copied()
            .unwrap();
        chars.next();
        let mut s = String::from(first);

        while let Some((_, c)) = chars.peek() {
            if c.is_ascii_digit() {
                s.push(*c);
                chars.next();
            } else {
                break;
            }
        }
        let span = Span {
            start,
            end: start + s.len(),
        };

        Literal::Int { value: s, span }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_stream, Literal, Span};

    #[test]
    fn it_parses_int_literal() {
        let mut chars = parse_stream("42069");
        let lit = Literal::parse(&mut chars);
        assert_eq!(
            lit,
            Literal::Int {
                value: "42069".to_owned(),
                span: Span { start: 0, end: 5 }
            }
        )
    }
}

fn main() {
    let s = "[1..5]";
}
