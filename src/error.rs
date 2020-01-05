#[derive(Debug)]
pub enum ErrorType {
    InvalidCommand,
    Memory,
    StmntEnd,
    Nom(nom::error::ErrorKind),
}

#[derive(Debug)]
pub struct ParseError<'a>(pub &'a [u8], pub ErrorType);

impl<'a> nom::error::ParseError<&'a [u8]> for ParseError<'a> {
    fn from_error_kind(input: &'a [u8], kind: nom::error::ErrorKind) -> Self {
        ParseError(input, ErrorType::Nom(kind))
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub type IResult<'a, T> = nom::IResult<&'a [u8], T, ParseError<'a>>;
