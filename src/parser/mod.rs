use crate::tokeniser::{Brackets, Keywords, LiteralKind, Operators, Token};

pub fn function_names(tokens: Vec<Token>) -> Vec<String> {
    let mut names = Vec::new();

    let mut iter = tokens.into_iter();

    while let Some(i) = iter.next() {
        if let Token::Keyword(Keywords::Fn) = i {
            if let Some(Token::Identifier(i)) = iter.next() {
                names.push(i.into());
            }
        }
    }

    names
}
