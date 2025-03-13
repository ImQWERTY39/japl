use crate::error::JAPLError;

mod token;
pub use token::Token;

mod keyword;
pub use keyword::Keyword;

mod literal;
pub use literal::Literal;

mod symbol;
pub use symbol::Symbol;

pub fn tokenise(file: &str) -> Result<Vec<Token>, JAPLError> {
    let mut tokens = Vec::new();
    let mut builder = String::new();

    let mut char_iter = file.chars().peekable();
    while let Some(cur_char) = char_iter.next() {
        if cur_char.is_whitespace() {
            if !builder.is_empty() {
                tokens.push(builder.as_str().try_into()?);
                builder.clear();
            }

            continue;
        }

        builder.push(cur_char);

        if Token::try_from(builder.as_str()).is_err() {
            if builder.len() == 1 {
                continue;
            }

            builder.pop();
            tokens.push(builder.as_str().try_into()?);
            builder.clear();
            builder.push(cur_char);
        }
    }

    if !builder.is_empty() {
        tokens.push(builder.as_str().try_into()?);
    }

    Ok(tokens)
}
