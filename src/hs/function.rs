use super::{expression::Expression, Error, Literal, ParseStream};
use crate::Statement;

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
    pub ident: String,
    pub patterns: Vec<Pattern>,
    pub stmt: Statement,
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
        let stmt = Statement::Expression(expr);
        Ok(Self {
            ident,
            patterns,
            stmt,
        })
    }
}
