#![warn(clippy::all)]

mod ast;
mod error;
mod parse;
mod position;
mod utils;

pub use ast::*;
pub use error::*;
use nom::IResult;
use nom_locate::LocatedSpan;
pub use parse::*;
pub use position::*;
pub(crate) use utils::*;

type Result<'a, T> = IResult<Span<'a>, T, ParsingError<'a>>;
type Span<'a> = LocatedSpan<&'a str>;
