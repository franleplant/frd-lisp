use lexer::*;

use std::collections::HashMap;

pub type Prods = HashMap<&'static str, Vec<Vec<&'static str>>>;

lazy_static! {
    static ref PRODS: Prods = {
        let mut map = HashMap::new();

        map.insert(
            "Program",
            vec![vec!["Expression"], vec!["Expression", "Program"]],
        );

        map.insert(
            "Expression",
            vec![vec!["(", ")"], vec!["(", "List", ")"], vec!["Atom"]],
        );

        map.insert("List", vec![vec!["Expression"], vec!["Expression", "List"]]);

        map.insert("Atom", vec![vec!["Id"], vec!["Num"], vec!["PrimitiveOp"]]);

        return map;
    };
}

#[derive(Debug)]
pub struct Tree {
    ntype: String,
    children: Children,
}

pub type Children = Vec<Child>;

#[derive(Debug)]
pub enum Child {
    Leaf(String),
    Tree(Tree),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    //prods: Prods,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            tokens: vec![],
            index: 0,
        };
    }

    pub fn parse(&mut self, input: &str) -> Result<Tree, String> {
        self.tokens = lex(input);
        self.index = 0;

        println!("PARSE {:?}", self.tokens);

        return self.process_non_terminal(&"Program".to_string());
    }

    fn process_non_terminal(&mut self, non_terminal: &str) -> Result<Tree, String> {
        let prods = PRODS.get(non_terminal).unwrap();
        let backtrack_pivot = self.index;


        for right_side in prods {
            println!("Prod attempt: {:?} -> {:?}", non_terminal, right_side);
            self.index = backtrack_pivot;
            match self.process_production(right_side) {
                Err(_) => {
                    println!("Prod attempt: {:?} -> {:?} FAILED", non_terminal, right_side);
                }
                Ok(children) => {
                    return Ok(Tree {
                        ntype: non_terminal.to_string(),
                        children: children,
                    })
                }
            }
        }
        println!("backtrack pivot out {:?}", backtrack_pivot);
        return Err("dont know".to_string());
    }

    fn process_production(&mut self, right_side: &Vec<&str>) -> Result<Children, String> {
        let mut children: Children = vec![];

        for &symbol in right_side {
            println!("current token {:?}", self.tokens[self.index].kind);
            if self.is_terminal(symbol) {
                println!("terminal {:?}", symbol);
                if symbol == self.tokens[self.index].kind {
                    children.push(Child::Leaf(symbol.to_string()));
                    self.index += 1;
                } else {
                    return Err("Unexpected Token".to_string());
                }
            } else {
                println!("non terminal {:?}", symbol);
                let sub_tree = self.process_non_terminal(symbol)?;
                children.push(Child::Tree(sub_tree))
            }
        }

        return Ok(children);
    }

    fn is_non_terminal(&self, symbol: &str) -> bool {
        match PRODS.get(symbol) {
            Some(_) => return true,
            _ => return false,
        }
    }

    fn is_terminal(&self, symbol: &str) -> bool {
        return !self.is_non_terminal(symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        let cases = vec![
                ("(begin)", r#"List([Atom(Symbol("begin"))])"#),
                ("(begin 1 2)", r#"List([Atom(Symbol("begin")), Atom(Number(1.0)), Atom(Number(2.0))])"#),
                ("((closure 1 2) 1 (list 1 2))", r#"List([List([Atom(Symbol("closure")), Atom(Number(1.0)), Atom(Number(2.0))]), Atom(Number(1.0)), List([Atom(Symbol("list")), Atom(Number(1.0)), Atom(Number(2.0))])])"#),

            ];

        for (program, expected) in cases {
            let ast = Parser::new().parse(program);
            let ast_string = format!("{:?}", ast);
            assert_eq!(ast_string, expected, "program {:?}", program)
        }
    }

    #[test]
    fn integration_case_1() {
        let program = "(begin (define r 10) (* pi (* r r)))";
        let ast = Parser::new().parse(program);

        let ast_string = format!("{:?}", ast);
        let expected_ast_string = r#"List([Atom(Symbol("begin")), List([Atom(Symbol("define")), Atom(Symbol("r")), Atom(Number(10.0))]), List([Atom(Symbol("*")), Atom(Symbol("pi")), List([Atom(Symbol("*")), Atom(Symbol("r")), Atom(Symbol("r"))])])])"#;

        assert_eq!(ast_string, expected_ast_string)
    }
}
