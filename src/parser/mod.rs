use crate::tokeniser::{Brackets, Keywords, LiteralKind, Operators, Token};

pub fn function_names(tokens: Vec<Token>) -> Vec<(String, Vec<Vec<Token>>, usize)> {
    let mut names = Vec::new();

    let mut iter = tokens.into_iter();

    while let Some(i) = iter.next() {
        if let Token::Keyword(Keywords::Fn) = i {
            let val = parse_function(&mut iter);
            names.push(val);
        }
    }

    names
}

fn parse_function(iter: &mut std::vec::IntoIter<Token>) -> (String, Vec<Vec<Token>>, usize) {
    let fname = if let Some(Token::Identifier(i)) = iter.next() {
        i.into()
    } else {
        return (String::from("Invalid func"), Vec::new(), 0);
    };

    let valid_func_def = iter
        .next()
        .is_some_and(|x| matches!(x, Token::Bracket(Brackets::OpenParen)));

    if !valid_func_def {
        return (String::from("Invalid func"), Vec::new(), 0);
    }

    let mut args = Vec::new();
    let mut cur_arg = Vec::new();
    while let Some(i) = iter.next() {
        let comma = matches!(i, Token::Operator(Operators::Comma));
        let close_paren = matches!(i, Token::Bracket(Brackets::CloseParen));
        let valid_token = match i {
            Token::Identifier(_)
            | Token::Operator(Operators::Comma)
            | Token::Bracket(Brackets::CloseParen) => true,
            Token::Keyword(ref i) => i.is_keyword(),
            _ => false,
        };

        if !valid_token {
            return (String::from("Invalid func"), Vec::new(), 0);
        }

        if comma || close_paren {
            args.push(cur_arg);
            cur_arg = Vec::new();

            if comma {
                continue;
            } else {
                break;
            }
        }

        cur_arg.push(i);
    }

    if !cur_arg.is_empty() {
        return (String::from("Invalid func"), Vec::new(), 0);
    }

    if iter
        .next()
        .is_some_and(|x| !matches!(x, Token::Bracket(Brackets::OpenCurly)))
    {
        return (String::from("Invalid func"), Vec::new(), 0);
    }

    let mut c: u16 = 1;
    let mut count = 0;
    while let Some(i) = iter.next() {
        if let Token::Bracket(Brackets::OpenCurly) = i {
            c += 1;
        } else if let Token::Bracket(Brackets::CloseCurly) = i {
            c -= 1;

            if c == 0 {
                break;
            }
        } else if let Token::Operator(Operators::End) = i {
            count += 1;
        }
    }

    if c != 0 {
        return (String::from("Invalid func"), Vec::new(), 0);
    }

    (fname, args, count)
}
