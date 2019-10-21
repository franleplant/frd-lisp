#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate lalrpop_util;
// synthesized by LALRPOP
lalrpop_mod!(
    #[allow(clippy)]
    grammar
);

use std::rc::Rc;

mod ast;
mod env;
mod eval;
mod intrinsics;
mod lisp_value;

use std::io;
use std::io::prelude::*;

fn input() -> String {
    print!("frd_lisp$");
    io::stdout().flush().unwrap();

    let mut reply = String::new();
    io::stdin().read_line(&mut reply).unwrap();
    reply
}

pub fn main() {
    println!("FRD LISP: REPL (interactive) MODE \n\n");
    let global_env = Rc::new(env::Env::new_global());

    //TODO use a real REPL crate for this
    loop {
        let line = input();
        println!(">>> {:?}", repl_eval(&line, global_env.clone())[0]);
    }
}

fn repl_eval(source: &str, env: Rc<env::Env>) -> Vec<Rc<lisp_value::LispValue>> {
    let parser = grammar::ProgramParser::new();
    let result = parser.parse(source);
    assert!(result.is_ok(), "Syntax error {:?}", result);

    let result = eval::eval_program(&result.unwrap(), env);

    return result;
}

#[test]
fn main_test() {}
