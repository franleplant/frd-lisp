#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

mod lexer;
mod parser;

fn main() {
    let _ = env_logger::try_init();

    let program = "(begin (define r 10) (* pi (* r r)))";
    println!("{:?}\n{:?}", program, parser::Parser::new().parse(program));
}
