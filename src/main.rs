use rhc::transpile;
use std::{fs::File, io::Read};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(path).unwrap();
    let mut source = String::new();
    f.read_to_string(&mut source).unwrap();

    let js = transpile(&source);
    println!("{}", js);
}
