use crate::constants::*;
use crate::errors::TokenizeError;

mod literal_kind;
use literal_kind::LiteralKind;

mod keywords;
use keywords::Keywords;

mod operators;
use operators::Operators;

mod brackets;
use brackets::Brackets;

#[derive(PartialEq, Debug)]
pub enum Token {
    Literal(LiteralKind),
    Identifier(StrPtr),
    Keyword(Keywords),
    Operator(Operators),
    Bracket(Brackets),
}

impl TryFrom<&str> for Token {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = LiteralKind::try_from(value) {
            return Ok(Token::Literal(i));
        }

        if let Ok(i) = Keywords::try_from(value) {
            return Ok(Token::Keyword(i));
        }

        if let Ok(i) = Operators::try_from(value) {
            return Ok(Token::Operator(i));
        }

        if let Ok(i) = Brackets::try_from(value) {
            return Ok(Token::Bracket(i));
        }

        if valid_identifier(value) {
            return Ok(Token::Identifier(value.into()));
        }

        Err(TokenizeError::InvalidToken(value.into()))
    }
}

fn valid_identifier(token: &str) -> bool {
    let mut valid_identifiers: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    valid_identifiers.push('_');

    if !valid_identifiers.contains(&token.chars().next().unwrap_or_default()) {
        return false;
    }

    valid_identifiers.extend('0'..='9');
    valid_identifiers.push('_');

    for i in token.chars() {
        if !valid_identifiers.contains(&i) {
            return false;
        }
    }

    true
}

pub fn tokenise(expr: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let mut in_str = false;
    let mut in_char = false;

    for i in expr.chars() {
        if i.is_whitespace() && !(in_str || in_char) {
            if let Ok(i) = Token::try_from(buffer.as_str()) {
                tokens.push(i);
                buffer.clear();
            }

            continue;
        }

        let is_already_valid = !buffer.is_empty()
            && Token::try_from(buffer.as_str()).is_ok()
            && buffer != "'"
            && buffer != "\"";

        if i == '"' && !in_char {
            in_str = !in_str;
        }

        if i == '\'' && !in_str {
            in_char = !in_char;
        }

        buffer.push(i);

        if Token::try_from(buffer.as_str()).is_err() {
            if (in_str || in_char) && !is_already_valid {
                continue;
            }

            buffer.pop();
            tokens.push(Token::try_from(buffer.as_str())?);
            buffer.clear();
            buffer.push(i);
        }

        if buffer == "-" {
            /* use this if needed cause idk what im doing
            tokens.last().is_some_and(|x| {
                matches!(x, Token::Literal(LiteralKind::Float(_)))
                    || matches!(x, Token::Literal(LiteralKind::Integer(_)))
                    || matches!(x, Token::Identifier(_))
            })
            */
            if tokens
                .last()
                .is_some_and(|x| matches!(x, Token::Operator(_)))
            {
                buffer.clear();
                tokens.push(Token::Operator(Operators::Sub));
            }
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::try_from(buffer.as_str())?);
    }

    Ok(tokens)
}
