#[macro_use]
extern crate lazy_static;

mod lexer;
mod parser;

use parser::parse;

fn main() {
    let program = "(begin (define r 10) (* pi (* r r)))";
    println!("{:?}\n{:?}", program, parse(program));
}
