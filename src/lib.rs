pub mod hs;
use hs::{Expression, Pattern};

mod write;
pub use write::Write;

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
    pub patterns: Vec<Pattern>,
    pub stmt: Statement,
}

pub struct FunctionItem {
    pub ident: String,
    pub blocks: Vec<Block>,
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
                    Pattern::Literal(_lit) => {}
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
                        block.stmt.to_js(s);
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
                        block.stmt.to_js(s);
                        s.push('}');
                    }
                }
            },
        );
    }
}
