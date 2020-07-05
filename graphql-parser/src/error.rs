use crate::{Result, Span};
use nom::{
    error::{ErrorKind, ParseError},
    Err::Error,
};

#[derive(Debug, PartialEq)]
pub enum ParsingError<'a> {
    Nom(Span<'a>, ErrorKind),
}

impl<'a> ParseError<Span<'a>> for ParsingError<'a> {
    fn from_error_kind(s: Span<'a>, kind: ErrorKind) -> Self {
        ParsingError::Nom(s, kind)
    }

    fn append(_: Span<'a>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[allow(dead_code)]
pub(crate) fn err<'a, O, F, E>(f: F, e: E) -> impl Fn(Span<'a>) -> Result<O>
where
    F: Fn(Span<'a>) -> Result<O>,
    E: Fn(Span<'a>, ErrorKind) -> ParsingError,
{
    move |s: Span<'a>| {
        f(s).map_err(|err| {
            if let Error(ParsingError::Nom(span, error_kind)) = err {
                Error(e(span, error_kind))
            } else {
                err
            }
        })
    }
}
