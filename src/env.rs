use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::intrinsics;
use crate::lisp_value::LispValue;

pub type Map = HashMap<String, Rc<LispValue>>;

#[derive(Debug)]
pub struct Env {
    root: Option<Rc<Env>>,
    parent: Option<Rc<Env>>,
    env: RefCell<Map>,
}

impl Env {
    pub fn new_global() -> Env {
        let mut env = HashMap::new();
        env.insert(
            "+".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::add)),
        );
        env.insert(
            "-".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::sub)),
        );
        env.insert(
            "=".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::eq)),
        );
        env.insert(
            ">".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::gt)),
        );
        env.insert(
            "<".to_string(),
            Rc::new(LispValue::Intrinsic(intrinsics::lt)),
        );

        Env {
            root: None,
            parent: None,
            env: RefCell::new(env),
        }
    }

    pub fn new(&self, parent: Rc<Env>, env: Map) -> Env {
        Env {
            root: self.root.clone(),
            parent: Some(parent),
            env: RefCell::new(env),
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<LispValue>> {
        {
            let env = self.env.borrow();
            let value = env.get(key);
            if let Some(inner_value) = value {
                return Some(inner_value.clone());
            }
        }

        self.parent.as_ref()?;

        self.parent.as_ref().unwrap().get(key)
    }

    pub fn set(&self, key: String, value: Rc<LispValue>) {
        self.env.borrow_mut().insert(key, value);
    }

    pub fn set_global(&self, key: String, value: Rc<LispValue>) {
        // This is the global env
        if self.root.is_none() {
            self.set(key, value);
            return;
        }

        self.root.as_ref().unwrap().set(key, value)
    }

    // TODO Maybe?
    //pub fn new_scope(parent: Rc<Map>, keys: Vec<String>, values: Vec<LispValue>)
}
