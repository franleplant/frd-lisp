use std::iter::FromIterator;

#[derive(Debug)]
enum NFAResult {
    Accepted,
    NotAccepted,
    Trapped,
}

fn nfa_par_open(src: &str) -> NFAResult {
    if src == "(" {
        return NFAResult::Accepted
    } else {
        return NFAResult::Trapped
    }
}

fn nfa_par_close(src: &str) -> NFAResult {
    if src == ")" {
        return NFAResult::Accepted
    } else {
        return NFAResult::Trapped
    }
}

#[derive(Debug, Clone)]
enum TokenKind {
    ParOpen,
    ParClose,
    Id,
    Num,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    lexeme: String,
}

fn lex(src: &str) -> Vec<Token> {

    let TOKEN_CONFIG: Vec<(TokenKind, fn(&str) -> NFAResult)> = vec![
        (TokenKind::ParOpen, nfa_par_open),
        (TokenKind::ParClose, nfa_par_close),
    ];

    let mut tokens: Vec<Token> = vec![];
    let mut index = 0;
    let mut chars: Vec<_> = src.chars().collect();
    chars.push(' ');

    //TODO use iterators with peekable?
    while index < chars.len() {
        let c = chars[index];
        if c.is_whitespace() {
            continue
        }


        let start = index;
        let mut all_trapped = false;
        let mut candidates = vec![];
        let mut next_candidates = vec![];
        while !all_trapped {
            println!("FUCK SHIT FUCK {}", index);
            all_trapped = true;
            candidates = next_candidates;
            next_candidates = vec![];

            for (token_kind, nfa) in &TOKEN_CONFIG {
                let lexeme = String::from_iter(&chars[start..index + 1]);
                println!("lexeme {}", lexeme);

                let res = nfa(&lexeme);
                match res {
                    // TODO or?
                    NFAResult::Accepted => {
                        all_trapped = false;
                        next_candidates.push(token_kind)
                    },
                    NFAResult::NotAccepted => { all_trapped = false; },
                    NFAResult::Trapped => { }
                }
            }

            index += 1;
        }
        println!("out {}", index);

        index -= 1;

        assert!(candidates.len() > 0, "asdjhasd");
        let lexeme = String::from_iter(&chars[start..index + 1]);
        let token_kind = candidates[0].clone();
        let token = Token { kind: token_kind, lexeme: lexeme.to_string()};
        println!("token {:?}", token);
        tokens.push(token)
    }

    return tokens
}


fn main() {
    println!("{:?}", lex("(()"));
}
