use grammar::{new_leaf_symbol, postprocess_tree, PRODS};
use lexer::{lex, Token};
use parse_node::{Children, Node};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    backtrack_id: usize,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            tokens: vec![],
            index: 0,
            backtrack_id: 0,
        };
    }

    pub fn get_backtrack_id(&mut self) -> usize {
        let backtrack_id = self.backtrack_id;
        self.backtrack_id += 1;
        return backtrack_id;
    }

    pub fn get_token(&self) -> &Token {
        return &self.tokens[self.index];
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

    pub fn parse(&mut self, input: &str) -> Result<Node, String> {
        self.tokens = lex(input);
        self.index = 0;

        debug!("PARSE {:?}", self.tokens);

        return self.process_non_terminal(&"Program".to_string());
    }

    fn process_non_terminal(&mut self, non_terminal: &str) -> Result<Node, String> {
        let prods = PRODS.get(non_terminal).unwrap();
        let backtrack_pivot = self.index;
        let backtrack_id = self.get_backtrack_id();

        for right_side in prods {
            debug!(
                ">>> {} Prod attempt: {:?} -> {:?}",
                backtrack_id, non_terminal, right_side
            );

            self.index = backtrack_pivot;
            match self.process_production(right_side) {
                Err(_) => {
                    debug!(
                        "<<< {} Prod attempt: {:?} -> {:?} FAILED",
                        backtrack_id, non_terminal, right_side
                    );
                }
                Ok(children) => {
                    debug!(
                        "<<< {} Prod attempt: {:?} -> {:?} SUCCEEDED",
                        backtrack_id, non_terminal, right_side
                    );
                    let sub_tree = Node::new_tree(non_terminal.to_string(), children);
                    let sub_tree = postprocess_tree(sub_tree, (non_terminal, right_side));

                    return Ok(sub_tree);
                }
            }
        }
        debug!("backtrack pivot out {:?}", backtrack_pivot);
        return Err(format!(
            "Cannot find the right derivation in token {:?}",
            self.tokens[self.index]
        ));
    }

    fn process_production(&mut self, right_side: &Vec<&str>) -> Result<Children, String> {
        let mut children: Children = vec![];
        debug!("+++ process the production {:?}", right_side);

        for &symbol in right_side {
            debug!("$$$ symbol {:?}", symbol);
            debug!("current token {:?}", self.get_token());

            if self.is_terminal(symbol) {
                debug!("terminal {:?}", symbol);
                if symbol == self.get_token().kind {
                    let leaf = new_leaf_symbol(self.get_token());
                    children.push(leaf);

                    self.index += 1;
                    debug!("NEXT SYMBOL");
                } else {
                    debug!("WRONG SYMBOL");
                    return Err("Unexpected Token".to_string());
                }
            } else {
                debug!("non terminal {:?}", symbol);
                let sub_tree = self.process_non_terminal(symbol)?;
                children.push(sub_tree);
            }
        }

        debug!("+++ OK process the production {:?}", right_side);
        return Ok(children);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cases() {
        let _ = env_logger::try_init();

        let cases = vec![
                //(
        //"(begin)",
        //r#"Tree { ntype: "Program", children: [Tree { ntype: "Expression", children: [Leaf("("), Tree { ntype: "List", children: [Tree { ntype: "Expression", children: [Tree { ntype: "Atom", children: [Leaf("begin")] }] }] }, Leaf(")")] }] }"#
        //),

                ("(begin 1 2)", r#"List([Atom(Symbol("begin")), Atom(Number(1.0)), Atom(Number(2.0))])"#),
                ("((closure 1 2) 1 (list 1 2))", r#"List([List([Atom(Symbol("closure")), Atom(Number(1.0)), Atom(Number(2.0))]), Atom(Number(1.0)), List([Atom(Symbol("list")), Atom(Number(1.0)), Atom(Number(2.0))])])"#),

            ];

        for (program, expected) in cases {
            let ast = Parser::new().parse(program).expect("To be parsed ok");
            let ast_string = format!("{:?}", ast);
            println!("Actual \n {:#?}", ast);
            assert_eq!(ast_string, expected, "program {:?}", program)
        }
    }

    #[test]
    fn integration_case_1() {
        let _ = env_logger::try_init();

        let program = "(begin (define r 10) (* pi (* r r)))";
        let ast = Parser::new().parse(program);

        let ast_string = format!("{:?}", ast);
        let expected_ast_string = r#"List([Atom(Symbol("begin")), List([Atom(Symbol("define")), Atom(Symbol("r")), Atom(Number(10.0))]), List([Atom(Symbol("*")), Atom(Symbol("pi")), List([Atom(Symbol("*")), Atom(Symbol("r")), Atom(Symbol("r"))])])])"#;

        assert_eq!(ast_string, expected_ast_string)
    }
}
