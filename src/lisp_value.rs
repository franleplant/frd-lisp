use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::Expr;
use crate::env::Env;
use crate::eval::eval_expression;

#[derive(Clone)]
pub enum LispValue {
    Nill,
    Id(String),
    Int(i64),
    Bool(Bool),
    Intrinsic(fn(&[Rc<LispValue>]) -> Rc<LispValue>),
    Func(Func),
}

impl LispValue {
    pub fn unwrap_number(&self) -> &i64 {
        match self {
            LispValue::Int(ref num) => num,
            _ => panic!("BBBB"),
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &LispValue) -> bool {
        use self::LispValue::*;

        match (self, other) {
            (LispValue::Nill, LispValue::Nill) => true,
            (Int(ref n1), Int(ref n2)) => n1 == n2,
            (Id(ref id1), Id(ref id2)) => *id1 == *id2,
            (Bool(ref bool1), Bool(ref bool2)) => bool1 == bool2,
            _ => false,
        }
    }
}

impl Eq for LispValue {}

impl Ord for LispValue {
    fn cmp(&self, other: &LispValue) -> Ordering {
        use self::LispValue::*;

        match (self, other) {
            (Int(ref n1), Int(ref n2)) => n1.cmp(n2),
            // TODO is this the right thing to do?
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for LispValue {
    fn partial_cmp(&self, other: &LispValue) -> Option<Ordering> {
        use self::LispValue::*;
        match (self, other) {
            (Int(ref n1), Int(ref n2)) => Some(n1.cmp(n2)),
            // TODO is this the right thing to do?
            _ => None,
        }
    }
}

impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::Nill => write!(f, "Nill"),
            LispValue::Intrinsic(_) => write!(f, "intrinsic"),
            LispValue::Func(func) => write!(f, "#func {}", func.get_name()),
            LispValue::Int(num) => write!(f, "{}", num),
            LispValue::Id(str) => write!(f, "{}", str),
            LispValue::Bool(lisp_bool) => match lisp_bool {
                Bool::True => write!(f, "true"),
                Bool::False => write!(f, "false"),
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}

#[derive(Clone)]
pub struct Func {
    name: String,
    arg_names: Vec<String>,
    body: Vec<Expr>,
    env: Rc<Env>,
}

impl Func {
    pub fn new(name: String, arg_names: Vec<String>, body: Vec<Expr>, env: Rc<Env>) -> Func {
        Func {
            name,
            arg_names,
            body,
            env,
        }
    }

    pub fn call(&self, arg_values: Vec<Rc<LispValue>>) -> Rc<LispValue> {
        let local_env: HashMap<String, Rc<LispValue>> =
            self.arg_names.clone().into_iter().zip(arg_values).collect();

        let env = Rc::new(self.env.new(self.env.clone(), local_env));

        // TODO evaluate multiple Expr bodies
        let result = eval_expression(&self.body[0], env.clone());
        debug!("func call {:?}", env);
        debug!("func result {:?}", result);
        result
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl Ord for Func {
    fn cmp(&self, _other: &Func) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for Func {
    fn partial_cmp(&self, _other: &Func) -> Option<Ordering> {
        None
    }
}

impl PartialEq for Func {
    fn eq(&self, _other: &Func) -> bool {
        false
    }
}

impl Eq for Func {}
