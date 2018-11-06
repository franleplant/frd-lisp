#[macro_use]
extern crate lazy_static;

mod lexer;
mod parser;

fn main() {
    let program = "(begin (define r 10) (* pi (* r r)))";
    println!("{:?}\n{:?}", program, parser::Parser::new().parse(program));
}
