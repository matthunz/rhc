use rhc::{
    hs::{FromTokens, Module, Tokens},
    js::{Block, FunctionItem},
};
use std::{fs::File, io::Read};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let mut tokens = Tokens::new(&s);

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
    println!("{}", js);
}
