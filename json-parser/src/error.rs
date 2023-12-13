use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Debug, PartialEq)]
pub enum ParserError<'a> {
    Cast(&'a str),
}

impl<'a> Display for ParserError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            ParserError::Cast(type_str) => write!(f, "cannot cast to {}", type_str),
        }
    }
}

impl<'a> Error for ParserError<'a> {}

pub type Result<'a, T> = std::result::Result<T, ParserError<'a>>;
