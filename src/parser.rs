use lexer::*;

#[derive(Debug)]
pub enum Expression {
    List(List),
    Atom(Atom),
}

// E -> ( L ) | A
impl Expression {
    fn new(tokens: &mut Vec<Token>) -> Expression {
        if tokens.len() == 0 {
            return Expression::List(List(vec![]));
        }

        if tokens.last().unwrap().is_kind(TokenKind::ParOpen) {
            tokens.pop().unwrap();

            let list = List::new(tokens);

            tokens.pop().unwrap();
            return Expression::List(list);
        }

        if tokens.last().unwrap().is_kind(TokenKind::ParClose) {
            panic!("Syntax Error: unexpected ')'")
        }

        return Expression::Atom(Atom::new(tokens));
    }
}

#[derive(Debug)]
pub struct List(Vec<Expression>);
// L -> E*
impl List {
    fn new(mut tokens: &mut Vec<Token>) -> List {
        let mut list = vec![];
        while !tokens
            .last()
            .expect(&format!(
                "Syntax Error: unexpected end of input: {:?}, after {:?}",
                tokens.last().unwrap(),
                list.last(),
            ))
            .is_kind(TokenKind::ParClose)
        {
            list.push(Expression::new(&mut tokens))
        }

        return List(list);
    }
}

#[derive(Debug)]
pub enum Atom {
    Symbol(String),
    Number(f64),
}

// A -> number | symbol
impl Atom {
    fn new(tokens: &mut Vec<Token>) -> Atom {
        match tokens.last().unwrap().kind {
            TokenKind::Num => {
                let token = tokens.pop().unwrap();
                let num = token.lexeme.parse::<f64>().unwrap();
                return Atom::Number(num);
            }

            TokenKind::Id | TokenKind::PrimitiveOp => {
                let token = tokens.pop().unwrap();
                return Atom::Symbol(token.lexeme.clone());
            }

            _ => panic!("Syntax Error in ATOM"),
        }
    }
}


pub fn parse(src: &str) -> Expression {
    let mut tokens = lex(src).into_iter().rev().collect();
    return Expression::new(&mut tokens);
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
            let ast = parse(program);
            let ast_string = format!("{:?}", ast);
            assert_eq!(ast_string, expected, "program {:?}", program)
        }
    }

    #[test]
    fn integration_case_1() {
        let program = "(begin (define r 10) (* pi (* r r)))";
        let ast = parse(program);

        let ast_string = format!("{:?}", ast);
        let expected_ast_string = r#"List([Atom(Symbol("begin")), List([Atom(Symbol("define")), Atom(Symbol("r")), Atom(Number(10.0))]), List([Atom(Symbol("*")), Atom(Symbol("pi")), List([Atom(Symbol("*")), Atom(Symbol("r")), Atom(Symbol("r"))])])])"#;

        assert_eq!(ast_string, expected_ast_string)
    }
}
