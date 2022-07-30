mod expr;
use expr::Expression;

use std::{
    fs::File,
    io::{BufRead, BufReader},
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
pub enum Literal {
    Int { value: String, span: Span },
}

impl Literal {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let (start, first) = if let Some((pos, c)) = chars.peek().copied() {
            if c.is_ascii_digit() {
                chars.next();
                (pos, c)
            } else {
                return Err(Error::new(Span::new(pos, pos)));
            }
        } else {
            return Err(Error::empty());
        };

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
            end: start + s.len() - 1,
        };

        Ok(Literal::Int { value: s, span })
    }

    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::Int { value, span: _ } => s.push_str(value),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct List {
    start: Literal,
    end: Option<Literal>,
    bracket: Span,
}

impl List {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let bracket_start = parse_char(chars, '[')?;
        let start = Literal::parse(chars)?;

        for _ in 0..2 {
            parse_char(chars, '.')?;
        }

        let end = Literal::parse(chars).ok();
        let bracket_end = parse_char(chars, ']')?;

        Ok(Self {
            start,
            end,
            bracket: Span {
                start: bracket_start,
                end: bracket_end,
            },
        })
    }

    pub fn to_js(&self, s: &mut String) {
        let start = match &self.start {
            Literal::Int { value, span } => value,
        };

        if let Some(literal) = &self.end {
            let end = match literal {
                Literal::Int { value, span } => value,
            };

            let js = format!(
                "{{
                pos: {},
                end: {},
                next() {{
                    if (this.pos < this.end) {{
                        return {{ done: false, value: this.pos++ }};
                    }} else {{
                        return {{ done: true }};
                    }}
                }}
            }}",
                start, end
            );
            s.push_str(&js);
        } else {
            let js = format!(
                "{{
                pos: {},
                next() {{
                    return {{ done: false, value: this.pos++ }};
                }}
            }}",
                start
            );
            s.push_str(&js);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Pattern {
    Literal(Literal),
    Ident(String),
}

impl Pattern {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        if let Ok(lit) = Literal::parse(chars) {
            // remove whitespace?
            chars.next();
            Ok(Self::Literal(lit))
        } else {
            let ident: String = chars.map(|(_, c)| c).take_while(|c| *c != ' ').collect();
            if ident.is_empty() {
                todo!()
            }
            Ok(Self::Ident(ident))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    ident: String,
    patterns: Vec<Pattern>,
    block: Vec<Statement>,
}

impl Function {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let ident: String = chars
            .take_while(|(_, c)| *c != ' ')
            .map(|(_, c)| c)
            .collect();
        if ident.is_empty() {
            todo!()
        }

        let mut patterns = Vec::new();
        let mut arg = String::new();

        loop {
            let pattern = Pattern::parse(chars)?;
            patterns.push(pattern);
            if chars.peek().map(|(_, c)| *c) == Some('=') {
                chars.next();
                break;
            }
        }

        if chars.next().unwrap().1 != ' ' {
            todo!()
        }

        let expr = Expression::parse(chars)?;
        let block = vec![Statement::Expression(expr)];
        Ok(Self {
            ident,
            patterns,
            block,
        })
    }

    pub fn to_js(&self, s: &mut String) {
        s.push_str("function ");
        s.push_str(&self.ident);

        s.push('(');
        /*
        for (pos, arg) in self.patterns.iter().enumerate() {
            s.push_str(arg);
            if pos < self.patterns.len() - 1 {
                s.push(',');
            }
        }
        */
        s.push(')');

        s.push('{');
        s.push_str("return ");
        for stmt in &self.block {
            match stmt {
                Statement::Expression(expr) => {
                    expr.to_js(s);
                }
            }
        }

        s.push(';');
        s.push('}');
    }
}

pub struct Block {
    pat: Pattern,
    exprs: Vec<Expression>,
}

pub struct FunctionItem {
    blocks: Vec<Block>,
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let f = File::open(path).unwrap();
    let mut lines = BufReader::new(f).lines();

    let mut funcs = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let mut chars = parse_stream(&line);
        let func = Function::parse(&mut chars).unwrap();
        funcs.push(func);
    }
    dbg!(&funcs);
}

#[cfg(test)]
mod tests {
    use crate::{parse_stream, List, Literal, Span};

    #[test]
    fn it_parses_int_literal() {
        let mut chars = parse_stream("42069");
        let lit = Literal::parse(&mut chars).unwrap();
        assert_eq!(
            lit,
            Literal::Int {
                value: "42069".to_owned(),
                span: Span { start: 0, end: 4 }
            }
        )
    }

    #[test]
    fn it_parses_list_range() {
        let mut chars = parse_stream("[1..5]");
        let list = List::parse(&mut chars).unwrap();
        assert_eq!(
            list,
            List {
                start: Literal::Int {
                    value: "1".to_owned(),
                    span: Span { start: 1, end: 1 }
                },
                end: Some(Literal::Int {
                    value: "5".to_owned(),
                    span: Span { start: 4, end: 4 }
                }),
                bracket: Span { start: 0, end: 5 }
            }
        )
    }
}
