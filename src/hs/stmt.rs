use super::Expression;
use crate::js::Write;

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
