pub mod hs;
use hs::{Expression, Pattern};

mod write;
pub use write::Write;

use crate::hs::{parse_stream, Function};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(Expression),
}

impl Statement {
    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::Expression(expr) => {
                s.write_return(|s| expr.to_js(s));
            }
        }
    }
}

pub struct Block {
    patterns: Vec<Pattern>,
    stmts: Vec<Statement>,
}

pub struct FunctionItem {
    ident: String,
    blocks: Vec<Block>,
}

impl FunctionItem {
    pub fn to_js(&self, s: &mut String) {
        let mut args = Vec::new();
        for block in &self.blocks {
            for pat in &block.patterns {
                match pat {
                    Pattern::Ident(ident) => {
                        if !args.contains(ident) {
                            args.push(ident.clone());
                        }
                    }
                    Pattern::Literal(lit) => {}
                }
            }
        }

        s.write_function(
            &self.ident,
            |s| {
                for arg in &args {
                    s.push_str(arg);
                }
            },
            |s| {
                for block in &self.blocks {
                    let conds: Vec<_> = block
                        .patterns
                        .iter()
                        .filter_map(|pat| match pat {
                            Pattern::Ident(_) => None,
                            Pattern::Literal(lit) => Some(lit),
                        })
                        .collect();

                    if conds.is_empty() {
                        for stmt in &block.stmts {
                            stmt.to_js(s);
                        }
                    } else {
                        s.push_str("if (");
                        for (pos, cond) in conds.iter().enumerate() {
                            s.push_str(&args[pos]);
                            s.push_str(" == ");
                            cond.to_js(s);

                            if pos < conds.len() - 1 {
                                s.push_str(" && ");
                            }
                        }
                        s.push_str("){");

                        for stmt in &block.stmts {
                            stmt.to_js(s);
                        }
                        s.push('}');
                    }
                }
            },
        );
    }
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
    let i = FunctionItem {
        ident: funcs.first().unwrap().ident.clone(),
        blocks: funcs
            .iter()
            .map(|func| Block {
                patterns: func.patterns.clone(),
                stmts: func.block.clone(),
            })
            .collect(),
    };

    let mut js = String::new();
    i.to_js(&mut js);
    println!("{}", js);
}

#[cfg(test)]
mod tests {
    use crate::{
        hs::{List, Literal, Span},
        parse_stream,
    };

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
