pub mod hs;
use crate::hs::{FromTokens, Module, Tokens};

pub mod js;
use crate::js::{Block, FunctionItem};

pub fn transpile(source: &str) -> String {
    let mut tokens = Tokens::new(&source);

    let m = Module::from_tokens(&mut tokens).unwrap();
    let i = FunctionItem {
        ident: m.funcs.first().unwrap().ident.clone(),
        blocks: m
            .funcs
            .iter()
            .map(|func| Block {
                patterns: func.patterns.clone(),
                stmt: func.stmt.clone(),
            })
            .collect(),
    };

    let mut js = String::new();
    i.to_js(&mut js);
    js
}
