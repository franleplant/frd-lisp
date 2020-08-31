//TODO
//- create a proper debug infra with a better strucure
//- probably we would want to generate an id for each function instance to see how they open and close
//- better structure for what and how we log it, arguments, return values, env changes, etc
//
//
//TODO
//- instead of Panic! on every error it would nice to a hace Result infra
//
use crate::lisp_value::{Bool, Func, LispValue};
use log::debug;
use std::rc::Rc;

use crate::ast::{Atom, Expr};
use crate::env::Env;
use crate::grammar;

pub fn eval(source: &str) -> Vec<Result<Rc<LispValue>, String>> {
    debug!("eval {:?}", source);
    let parser = grammar::ProgramParser::new();
    let result = parser.parse(source);
    assert!(result.is_ok(), "Syntax error {:?}", result);
    debug!("ast {:?}", result);

    let global_env = Rc::new(Env::new_global());
    let result = eval_program(&result.unwrap(), global_env.clone());
    debug!("env {:?}", global_env);
    debug!("result {:?}", result);

    result
}

pub fn eval_program(program: &[Expr], env: Rc<Env>) -> Vec<Result<Rc<LispValue>, String>> {
    debug!("eval {:?}", program);

    let result: Vec<Result<Rc<LispValue>, String>> = program
        .iter()
        .map(|expr| eval_expression(expr, env.clone()))
        .collect();

    debug!("eval_program END");
    result
}

pub fn eval_expression(expression: &Expr, env: Rc<Env>) -> Result<Rc<LispValue>, String> {
    debug!("eval_expression {:?}", expression);

    let result = match expression {
        Expr::List(list) => eval_list(list, env),

        Expr::Atom(atom) => eval_atom(atom, env),

        Expr::DefineFunction(fn_name, arg_names, body) => {
            eval_define_function(fn_name.clone(), arg_names.clone(), body.clone(), env)
        }

        Expr::DefineVariable(name, value) => eval_define_variable(name, value, env),

        Expr::If(cond, positive, negative) => eval_if(cond, positive, negative, env),
    };

    debug!("eval_expression END {:?}", result);
    result
}

pub fn eval_list(list: &[Expr], env: Rc<Env>) -> Result<Rc<LispValue>, String> {
    debug!("eval_list {:?}", list);
    if list.is_empty() {
        return Ok(Rc::new(LispValue::Nill));
    }

    let mut list = list.to_vec();
    let first = list.remove(0);

    match first {
        Expr::Atom(atom) => {
            //TODO

            let id = if let Atom::Id(id) = atom {
                id
            } else {
                return Err(format!("unexpected non id {:?}", atom));
            };

            let func = env.get(&id).ok_or(format!("Symbol `{}` not found", id))?;

            let mut arg_values: Vec<Rc<LispValue>> = vec![];
            for expression_result in list.iter().map(|expr| eval_expression(expr, env.clone())) {
                arg_values.push(expression_result?);
            }

            match *func {
                LispValue::Intrinsic(ref func) => {
                    //TODO instrincis need to return a Result
                    let res = func(&arg_values);
                    debug!("eval_list END Intrinsice {:?}", res);
                    Ok(res)
                }

                LispValue::Func(ref func) => {
                    let res = func.call(arg_values);
                    debug!("eval_list END FUNC {:?}", res);
                    res
                }
                _ => Err("Unexpected Value in the Function name position".to_string()),
            }
        }
        //Expr::List(ref list) =>  {
        // evaluate the first element as a list, check what it evaluates to
        // and do something
        //let first = eval_list()
        //}
        _ => Err("Unhandled".to_string()),
    }
}

pub fn eval_atom(atom: &Atom, env: Rc<Env>) -> Result<Rc<LispValue>, String> {
    debug!("eval_atom {:?}", atom);
    match atom {
        Atom::Int(num) => Ok(Rc::new(LispValue::Int(*num))),
        Atom::Id(id) => match id.as_str() {
            "true" => Ok(Rc::new(LispValue::Bool(Bool::True))),
            "false" => Ok(Rc::new(LispValue::Bool(Bool::False))),
            _ => env.get(&id).ok_or(format!("Symbol {} not found", id)),
        },
    }
}

pub fn eval_define_function(
    fn_name: String,
    arg_names: Vec<String>,
    body: Vec<Expr>,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, String> {
    let func = Func::new(fn_name, arg_names, body, env.clone());
    env.set(func.get_name().clone(), Rc::new(LispValue::Func(func)));

    Ok(Rc::new(LispValue::Nill))
}

pub fn eval_define_variable(
    var_name: &str,
    var_value: &Expr,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, String> {
    let value = eval_expression(var_value, env.clone())?;
    env.set(var_name.to_string(), value);

    Ok(Rc::new(LispValue::Nill))
}

pub fn eval_if(
    cond: &Expr,
    positive: &Expr,
    negative: &Option<Expr>,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, String> {
    let cond_value = eval_expression(cond, env.clone())?;
    if let LispValue::Bool(ref value) = *cond_value {
        match value {
            Bool::True => eval_expression(positive, env),
            Bool::False => {
                if negative.is_none() {
                    return Ok(Rc::new(LispValue::Nill));
                }

                eval_expression(negative.as_ref().unwrap(), env)
            }
        }
    } else {
        //TODO
        Err("Still don\'t know how to coerce".to_string())
    }
}

//#[cfg(test)]
//mod tests {
//use super::*;

//#[test]
//fn base_cases() {
//let _ = env_logger::try_init();
//let ast = Parser::new().parse("(+ 1 2)").expect("To be parsed ok");
//let ast_string = format!("{:?}", ast);
//println!("Actual \n {:#?}", ast);
//let results = eval(&ast);
//assert_eq!(results[0].clone().unwrap_number(), 3.0);
//}
//}
