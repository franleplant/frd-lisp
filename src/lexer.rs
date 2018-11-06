use std::iter::FromIterator;

#[derive(Debug)]
pub enum NFAResult {
    Accepted,
    NotAccepted,
    Trapped,
}


type TokenKind = String;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn is_kind(&self, kind: TokenKind) -> bool {
        return self.kind == kind;
    }
}

lazy_static! {
    static ref TOKEN_CONFIG: Vec<(TokenKind, fn(&str) -> NFAResult)> = vec![
        ("ParOpen".to_string(), nfa_par_open),
        ("ParClose".to_string(), nfa_par_close),
        ("Id".to_string(), nfa_id),
        ("Num".to_string(), nfa_num),
        ("PrimitiveOp".to_string(), nfa_primitive_op),
    ];
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut index = 0;
    let mut chars: Vec<_> = src.chars().collect();
    chars.push(' ');

    //TODO use iterators with peekable?
    while index < chars.len() {
        let c = chars[index];
        if c.is_whitespace() {
            index += 1;
            continue;
        }

        let start = index;
        let mut all_trapped = false;
        let mut candidates = vec![];
        let mut next_candidates = vec![];
        let mut lexeme = String::new();
        let mut next_lexeme = String::new();
        while !all_trapped {
            all_trapped = true;
            candidates = next_candidates;
            next_candidates = vec![];
            lexeme = next_lexeme;
            next_lexeme = String::from_iter(&chars[start..index + 1]);

            //println!("lexeme {}", lexeme);
            //println!("next_lexeme {}", next_lexeme);

            for (token_kind, nfa) in TOKEN_CONFIG.iter() {
                let res = nfa(&next_lexeme);
                match res {
                    NFAResult::Accepted => {
                        all_trapped = false;
                        next_candidates.push(token_kind)
                    }
                    NFAResult::NotAccepted => {
                        all_trapped = false;
                    }
                    NFAResult::Trapped => {}
                }
            }

            index += 1;
        }

        index -= 1;

        assert!(
            candidates.len() > 0,
            "Unknown Token {:?} at {} {:?}",
            lexeme,
            index,
            c
        );
        let token_kind = candidates[0].clone();
        let token = Token {
            kind: token_kind,
            lexeme: lexeme.to_string(),
        };
        tokens.push(token);
    }

    return tokens;
}

fn nfa_par_open(src: &str) -> NFAResult {
    if src == "(" {
        return NFAResult::Accepted;
    } else {
        return NFAResult::Trapped;
    }
}

fn nfa_par_close(src: &str) -> NFAResult {
    if src == ")" {
        return NFAResult::Accepted;
    } else {
        return NFAResult::Trapped;
    }
}

fn nfa_id(src: &str) -> NFAResult {
    let mut state = 0;
    let accepted = [1];
    for c in src.chars() {
        match state {
            0 if c.is_alphabetic() => state = 1,
            1 if c.is_alphanumeric() => state = 1,
            _ => state = -1,
        }
    }

    if state == -1 {
        return NFAResult::Trapped;
    }

    if accepted.contains(&state) {
        return NFAResult::Accepted;
    } else {
        return NFAResult::NotAccepted;
    }
}

//TODO whatabout defyining accepted and the thing inside the match
//as the only param to to this shit?
fn nfa_num(src: &str) -> NFAResult {
    let mut state = 0;
    let accepted = [1, 3];
    for c in src.chars() {
        match state {
            0 if c.is_digit(10) => state = 1,
            1 if c.is_digit(10) => state = 1,
            1 if c == ',' => state = 2,
            2 if c.is_digit(10) => state = 3,
            _ => state = -1,
        }
        //if state == 0 && c.is_digit(10) {
        //state = 1;
        //} else if state == 1 && c.is_digit(10) {
        //state = 1;
        //} else if state == 1 && c == ',' {
        //state = 2;
        //} else if state == 2 && c.is_digit(10) {
        //state = 3;
        //} else {
        //state = -1;
        //break
        //}
    }

    if state == -1 {
        return NFAResult::Trapped;
    }

    if accepted.contains(&state) {
        return NFAResult::Accepted;
    } else {
        return NFAResult::NotAccepted;
    }
}

fn nfa_primitive_op(src: &str) -> NFAResult {
    let mut state = 0;
    let accepted = [1];
    for c in src.chars() {
        match state {
            0 => {
                if c == '+' || c == '-' || c == '*' || c == '/' {
                    state = 1;
                } else {
                    state = -1;
                }
            }
            _ => state = -1,
        }
    }

    if state == -1 {
        return NFAResult::Trapped;
    }

    if accepted.contains(&state) {
        return NFAResult::Accepted;
    } else {
        return NFAResult::NotAccepted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let tokens = lex("(() abc123 123 +");
        let tokens_string = format!("{:?}", tokens);
        let expected = r#"[Token { kind: ParOpen, lexeme: "(" }, Token { kind: ParOpen, lexeme: "(" }, Token { kind: ParClose, lexeme: ")" }, Token { kind: Id, lexeme: "abc123" }, Token { kind: Num, lexeme: "123" }, Token { kind: PrimitiveOp, lexeme: "+" }]"#;
        assert_eq!(tokens_string, expected);
    }
}
