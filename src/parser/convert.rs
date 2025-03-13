use crate::alias::{Name, Str};
use crate::error::JAPLError;
use crate::lexer::Token;
use crate::runtime::{RegisterName, Type};

pub fn get_register_name(tkn: Option<Token>) -> Result<RegisterName, JAPLError> {
    tkn.ok_or(JAPLError::IncorrectArgument(
        "Missing token: RegisterName".into(),
    ))
    .map(|tkn| {
        let err = JAPLError::IncorrectArgument("Exptected keyword: Register Name".into());

        if let Token::Keyword(kw) = tkn {
            RegisterName::try_from(kw).map_err(|_| err)
        } else {
            Err(err)
        }
    })?
}

pub fn get_variable_type(tkn: Option<Token>) -> Result<Type, JAPLError> {
    tkn.ok_or(JAPLError::IncorrectArgument(
        "Missing token: Variable Type".into(),
    ))
    .map(|tkn| {
        let err = JAPLError::IncorrectArgument("Exptected keyword: Variable Type".into());

        if let Token::Keyword(kw) = tkn {
            Type::try_from(kw).map_err(|_| err)
        } else {
            Err(err)
        }
    })?
}

pub fn get_ident_name(tkn: Option<Token>) -> Result<Name, JAPLError> {
    tkn.ok_or(JAPLError::IncorrectArgument(
        "Missing token: Identifier".into(),
    ))
    .map(|tkn| {
        let err = JAPLError::IncorrectArgument("Exptected identifier: Variable Name".into());

        if let Token::Identifier(ident) = tkn {
            Ok(Name::from(ident))
        } else {
            Err(err)
        }
    })?
}

pub fn get_label_name(tkn: Option<Token>) -> Result<Str, JAPLError> {
    tkn.ok_or(JAPLError::IncorrectArgument(
        "Missing token: Label Name".into(),
    ))
    .map(|tkn| {
        let err = JAPLError::IncorrectArgument("Exptected identifier: Label Name".into());

        if let Token::Identifier(ident) = tkn {
            Ok(ident)
        } else {
            Err(err)
        }
    })?
}
