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

        map.insert("List", vec![vec!["Expression", "List"], vec!["Expression"]]);

        map.insert("Atom", vec![vec!["Id"], vec!["Num"], vec!["PrimitiveOp"]]);

        return map;
    };
}

#[derive(Debug, Clone)]
pub struct Node {
    ntype: String,
    lexeme: String,
    children: Children,
}

pub type Children = Vec<Node>;

impl Node {
    pub fn new_tree(ntype: String, children: Children) -> Node {
        return Node {
            ntype,
            children,
            lexeme: String::new(),
        };
    }

    pub fn new_leaf(lexeme: String) -> Node {
        return Node {
            ntype: "Leaf".to_string(),
            children: vec![],
            lexeme,
        };
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    backtrack_id: usize,
    //prods: Prods,
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
                    let mut sub_tree = Node::new_tree(non_terminal.to_string(), children);

                    // TODO abstract this into soemthing configurable such as PRODs
                    // TODO do the same for Expression Program
                    // TODO maybe convert number strings into numbers? and any other data
                    // conversions?
                    if non_terminal == "List" && *right_side == vec!["Expression", "List"] {
                        sub_tree = flatten_list(sub_tree);
                    }

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
                    children.push(Node::new_leaf(self.get_token().lexeme.to_string()));
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

pub fn flatten_list(node: Node) -> Node {
    let flat_children = node
        .children
        .into_iter()
        .enumerate()
        .flat_map(|(i, node)| {
            if i == 1 && node.ntype == "List" {
                return node.children;
            }
            return vec![node];
        })
        //.map(|(i, tree)| Node::Tree(tree))
        .collect();

    return Node::new_tree(node.ntype, flat_children);
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
