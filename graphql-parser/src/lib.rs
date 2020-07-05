mod ast;
mod error;
mod position;
mod schema;
mod utils;

pub use ast::*;
pub use error::*;
use nom::IResult;
use nom_locate::LocatedSpan;
pub use position::*;
pub use schema::*;
pub(crate) use utils::*;

type Result<'a, T> = IResult<Span<'a>, T, ParsingError<'a>>;
type Span<'a> = LocatedSpan<&'a str>;
