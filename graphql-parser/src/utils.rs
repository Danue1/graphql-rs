use crate::*;
use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_till},
    character::complete::char,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::tuple,
};

pub(crate) fn name(s: Span) -> Result<String> {
    map(
        tuple((
            alt((alphabet, tag("_"))),
            many0(alt((alphabet, digit, tag("_")))),
        )),
        |(first, tail)| {
            let tail: String = tail.iter().map(|s| s.fragment().to_owned()).collect();
            format!("{}{}", first, tail)
        },
    )(s)
}

pub(crate) fn description(is_extend: bool) -> impl Fn(Span) -> Result<Option<Positioned<String>>> {
    move |s: Span| {
        if is_extend {
            Ok((s, None))
        } else {
            opt(map(
                tuple((
                    opt(hashtag_description),
                    ignore_token0,
                    positioned(multiline_description),
                    ignore_token0,
                    opt(hashtag_description),
                    ignore_token0,
                )),
                |(_, _, description, _, _, _)| description,
            ))(s)
        }
    }
}

pub(crate) fn multiline_description(s: Span) -> Result<String> {
    map(
        tuple((tag(r#"""""#), is_not(r#"""""#), tag(r#"""""#))),
        |(_, string, _): (Span, Span, Span)| string.fragment().to_string(),
    )(s)
}

pub(crate) fn hashtag_description(s: Span) -> Result<()> {
    map(
        tuple((
            hashtag,
            tuple((
                position,
                map(take_till(is_line_ending), |description: Span| {
                    description.fragment().to_string()
                }),
                position,
            )),
        )),
        |_| (),
    )(s)
}

pub(crate) fn string(s: Span) -> Result<String> {
    map(
        tuple((double_quote, opt(is_not(r#""\r\n"#)), double_quote)),
        |(_, string, _): ((), Option<Span>, ())| {
            string
                .map(|s| s.fragment().to_string())
                .unwrap_or_else(|| "".to_owned())
        },
    )(s)
}

pub(crate) fn is_nonzero_digit(c: char) -> bool {
    matches!(c, '1'..='9')
}

pub(crate) fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}

pub(crate) fn digit(s: Span) -> Result<Span> {
    is_a("0123456789")(s)
}

pub(crate) fn is_line_ending(c: char) -> bool {
    matches!(c, '\n' | '\r')
}

pub(crate) fn ignore_token0(s: Span) -> Result<()> {
    map(many0(is_a(", \t\r\n")), |_| ())(s)
}

pub(crate) fn ignore_token1(s: Span) -> Result<()> {
    map(
        many1(alt((map(is_a(", \t\r\n"), |_| ()), hashtag_description))),
        |_| (),
    )(s)
}

pub(crate) fn char_empty(c: char) -> impl Fn(Span) -> Result<()> {
    move |s: Span| map(char(c), |_| ())(s)
}

macro_rules! char_empty {
    ($($expr:expr => $ident:ident,)+) => {
        $(
            pub(crate) fn $ident(s: Span) -> Result<()> {
                char_empty($expr)(s)
            }
        )+
    };
}

char_empty!(
  '(' => left_parens,
  ')' => right_parens,
  '[' => left_bracket,
  ']' => right_bracket,
  '{' => left_brace,
  '}' => right_brace,
  '#' => hashtag,
  '.' => dot,
  ':' => colon,
  '+' => plus,
  '-' => hyphen,
  '=' => equal,
  '@' => at,
  '&' => ampersand,
  '!' => exclamation,
  '|' => pipeline,
  '"' => double_quote,
);

pub(crate) fn alphabet(s: Span) -> Result<Span> {
    is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)
}
