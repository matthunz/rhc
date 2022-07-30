use crate::{Error, Literal, ParseStream};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    BinaryOp {
        left: Box<Self>,
        op: BinaryOp,
        right: Box<Self>,
    },
    Path(Vec<String>),
    Literal(Literal),
    Call {
        ident: String,
        args: Vec<Self>,
    },
}

impl Expression {
    pub fn parse(chars: &mut ParseStream) -> Result<Self, Error> {
        let left = if let Ok(lit) = Literal::parse(chars) {
            Self::Literal(lit)
        } else {
            let part = chars.map(|(_, c)| c).take_while(|c| *c != ' ').collect();

            if chars.peek().map(|(_, c)| *c) == Some('(') {
                chars.next();

                let mut args = Vec::new();
                loop {
                    let arg = Self::parse(chars)?;
                    args.push(arg);
                    if chars.peek().map(|(_, c)| *c) == Some(')') {
                        chars.next();
                        // Remove space
                chars.next();
                        break;
                    }
                }

                Self::Call { ident: part, args }
            } else {
                Self::Path(vec![part])
            }
        };

        match chars.peek().map(|(_, c)| *c) {
            Some('+') => {
                chars.next();
                // Remove space
                chars.next();

                let right = Self::parse(chars)?;
                Ok(Self::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Add,
                    right: Box::new(right),
                })
            }
            Some('-') => {
                chars.next();
                // Remove space
                chars.next();

                let right = Self::parse(chars)?;
                Ok(Self::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Sub,
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    pub fn to_js(&self, s: &mut String) {
        match self {
            Self::BinaryOp { left, op, right } => {
                left.to_js(s);
                s.push('+');
                right.to_js(s);
            }
            Self::Literal(lit) => lit.to_js(s),
            Self::Path(path) => {
                for part in path {
                    s.push_str(part)
                }
            }
            _ => todo!(),
        }
    }
}
