use op;
use parse_node::{Node, SymbolType};
use parser::Parser;

#[derive(Clone)]
pub enum LispType {
    None,
    Num(f64),
    Fn(fn(&Vec<LispType>) -> LispType),
}

impl LispType {
    pub fn unwrap_number(self) -> f64 {
        match self {
            LispType::Num(num) => return num,
            _ => panic!("BBBB"),
        }
    }
    pub fn unwrap_fn(self) -> fn(&Vec<LispType>) -> LispType {
        match self {
            LispType::Fn(function) => return function,
            _ => panic!("BBBB"),
        }
    }
}

use std::fmt;
impl fmt::Debug for LispType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispType::None => write!(f, "Nill"),
            LispType::Fn(_) => write!(f, "Fn"),
            LispType::Num(num) => write!(f, "{}", num),
            _ => panic!("asdasd"),
        }
    }
}

pub fn eval(ast: &Node) -> Vec<LispType> {
    debug!("eval {:?}", ast.ntype);
    assert!(ast.ntype == "Program");

    // TODO this ast needs more flattening, it makes no sense to have List and I think Atom
    // either and also Delimiters
    let result: Vec<LispType> = ast
        .children
        .iter()
        .map(|child| eval_expression(child))
        .collect();

    for res in &result {
        println!("RESULT {:?}", res)
    }

    debug!("eval_program END");

    return result;
}

pub fn eval_expression(ast: &Node) -> LispType {
    debug!("eval_expression {:?}", ast.ntype);
    // Empty lists
    if ast.children.len() == 0 {
        return LispType::None;
    }

    let mut result: Vec<LispType> = ast
        .children
        .iter()
        .map(|child| match child.ntype.as_str() {
            "List" => {
                return eval_list(child);
            }

            "Atom" => {
                return eval_atom(child);
            }
            _ => panic!("UUUUU"),
        }).collect();

    assert!(result.len() == 1, "{:?}", result);

    debug!("eval_expression END");

    return result.pop().unwrap();
}

pub fn eval_list(ast: &Node) -> LispType {
    debug!("eval_list {:?}", ast.ntype);
    let mut parts: Vec<LispType> = ast
        .children
        .iter()
        .map(|child| eval_expression(child))
        .collect();
    let func = parts.remove(0);
    let func = func.unwrap_fn();
    let arguments = parts;
    let res = func(&arguments);
    debug!("eval_list END");
    return res;
}

pub fn eval_atom(ast: &Node) -> LispType {
    debug!("eval_atom {:?}", ast);
    let leaf = ast.children[0].clone();
    match leaf.symbol_type {
        SymbolType::Delimiter(_) => LispType::None,
        SymbolType::Num(num) => LispType::Num(num),
        SymbolType::Symbol(id) => {
            // TODO this should look in the env for information
            match id.as_str() {
                "+" => return LispType::Fn(op::add),
                _ => panic!("WE DONT HAVE ENVS YET DOg"),
            }
        }
        _ => panic!("I DON'T KNOW WHAT TUDU"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        let _ = env_logger::try_init();
        let ast = Parser::new().parse("(+ 1 2)").expect("To be parsed ok");
        let ast_string = format!("{:?}", ast);
        println!("Actual \n {:#?}", ast);
        let results = eval(&ast);
        assert_eq!(results[0].clone().unwrap_number(), 3.0);
    }
}
