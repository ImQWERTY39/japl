use crate::constants::*;
use crate::errors::TokenizeError;

#[derive(PartialEq, Debug)]
pub enum Token {
    Literal(LiteralKind),
    Identifier(StrPtr),
    Keyword(StrPtr),
    Operator(StrPtr),
    Bracket(StrPtr),
}

#[derive(PartialEq, Debug)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    Character(char),
    String(StrPtr),
}

impl TryFrom<&str> for Token {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = LiteralKind::try_from(value) {
            return Ok(Token::Literal(i));
        }

        if KEYWORDS.contains(&value) {
            return Ok(Token::Keyword(value.into()));
        }

        if OPERATORS.contains(&value) {
            return Ok(Token::Operator(value.into()));
        }

        if BRACKETS.contains(&value) {
            return Ok(Token::Bracket(value.into()));
        }

        if valid_identifier(value) {
            return Ok(Token::Identifier(value.into()));
        }

        Err(TokenizeError::InvalidToken(value.into()))
    }
}

impl TryFrom<&str> for LiteralKind {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<i64>() {
            return Ok(LiteralKind::Integer(i));
        }

        if let Ok(i) = value.parse::<f64>() {
            return Ok(LiteralKind::Float(i));
        }

        if value.starts_with('\'') && value.ends_with('\'') && value.len() == 3 {
            return Ok(LiteralKind::Character(
                value
                    .chars()
                    .nth(1)
                    .expect("Length has to be atleast 3, so index 1 exists"),
            ));
        }

        if value.starts_with("\'\\") && value.ends_with('\'') && value.len() == 4 {
            return Ok(LiteralKind::Character(as_escaped(
                value
                    .chars()
                    .nth(2)
                    .expect("Length is 4, so index 2 exists"),
            )?));
        }

        if value.starts_with('"') && value.ends_with('"') && value.len() > 2 {
            return Ok(LiteralKind::String(value[1..value.len() - 1].into()));
        }

        Err(TokenizeError::InvalidLiteral(value.into()))
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

fn as_escaped(escape_char: char) -> Result<char, TokenizeError> {
    match escape_char {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        '0' => Ok('\0'),
        error_chr => {
            let mut invalid_token = String::from("\\");
            invalid_token.push(error_chr);
            Err(TokenizeError::InvalidLiteral(
                invalid_token.into_boxed_str(),
            ))
        }
    }
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
                tokens.push(Token::Operator("-".into()));
            }
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::try_from(buffer.as_str())?);
    }

    Ok(tokens)
}
