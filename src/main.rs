use lalrpop_util::lalrpop_mod;

// synthesized by LALRPOP
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
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
        //TODO beeter error
        println!(">>> {:?}", repl_eval(&line, global_env.clone()));
    }
}

fn repl_eval(source: &str, env: Rc<env::Env>) -> Vec<Result<Rc<lisp_value::LispValue>, String>> {
    let parser = grammar::ProgramParser::new();
    let result = parser.parse(source);
    assert!(result.is_ok(), "Syntax error {:?}", result);

    eval::eval_program(&result.unwrap(), env)
}

#[test]
fn main_test() {}
