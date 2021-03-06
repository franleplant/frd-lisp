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
pub mod lisp_value;

// TODO move elsewhere
pub fn eval_file(file_name: &str) -> Vec<Result<Rc<lisp_value::LispValue>, String>> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    eval::eval(&contents)
}

#[test]
fn main_test() {
    let _ = env_logger::try_init();

    fn eval_with_debugs(source: &str) -> Vec<Result<Rc<lisp_value::LispValue>, String>> {
        println!("SOURCE {:?}", source);
        // PARSE
        let parser = grammar::ProgramParser::new();
        let result = parser.parse(source);
        println!("AST {:?}", result);
        assert!(result.is_ok());

        // Eval
        let global_env = Rc::new(env::Env::new_global());
        let result = eval::eval_program(&result.unwrap(), global_env.clone());
        println!("RESULT {:?}", result);
        println!("GLOBAL ENV {:#?}", global_env);

        result
    }

    let sources = vec![
        "(+ 1 2) (+ 3 2)",
        "(= true true)",
        "(define (myFn x) (+ x 2))",
        "(define (myFn x) (+ x 2)) (myFn 4)",
        "(define (my-fn-2? x) (+ x 2))",
        "(define myVar (+ 3 2))",
        "(if (= true true) (+ 1 2) (+ 3 4))",
    ];

    for source in &sources {
        eval_with_debugs(source);
    }
}
