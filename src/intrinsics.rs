use std::cmp::Ordering;
use std::rc::Rc;

use crate::lisp_value::{Bool, LispValue};

pub fn add(arguments: &[Rc<LispValue>]) -> Rc<LispValue> {
    let res = arguments.iter().fold(0, |acc, x| acc + x.unwrap_number());

    return Rc::new(LispValue::Int(res));
}

pub fn sub(arguments: &[Rc<LispValue>]) -> Rc<LispValue> {
    let first = arguments[0].unwrap_number();

    let res = arguments
        .iter()
        .skip(1)
        .fold(*first, |acc, x| acc - x.unwrap_number());

    return Rc::new(LispValue::Int(res));
}

pub fn eq(arguments: &[Rc<LispValue>]) -> Rc<LispValue> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        if !first.eq(left_hand) {
            return Rc::new(LispValue::Bool(Bool::False));
        }
    }

    return Rc::new(LispValue::Bool(Bool::True));
}

pub fn gt(arguments: &[Rc<LispValue>]) -> Rc<LispValue> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        match first.cmp(left_hand) {
            Ordering::Greater => {}
            _ => return Rc::new(LispValue::Bool(Bool::False)),
        }
    }
    return Rc::new(LispValue::Bool(Bool::True));
}

pub fn lt(arguments: &[Rc<LispValue>]) -> Rc<LispValue> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        match first.cmp(left_hand) {
            Ordering::Less => {}
            _ => return Rc::new(LispValue::Bool(Bool::False)),
        }
    }
    return Rc::new(LispValue::Bool(Bool::True));
}
