use crate::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1},
    character::complete::char,
    combinator::{all_consuming, map, opt},
    error::ErrorKind,
    multi::{many0, many1, separated_list},
    sequence::tuple,
    Err::Error,
};
use std::collections::BTreeMap;

const UNEXTEND: bool = false;

pub fn parse_schema(source: &str) -> std::result::Result<Document, ParsingError> {
    match all_consuming(map(
        tuple((
            many0(map(
                tuple((ignore_token0, positioned(definition))),
                |(_, definition)| definition,
            )),
            ignore_token0,
        )),
        |(definition_list, _)| definition_list,
    ))(LocatedSpan::new(source))
    {
        Ok((_, definition_list)) => Ok(Document { definition_list }),
        Err(Error(error)) => Err(error),
        _ => std::unreachable!(),
    }
}

fn extend(s: Span) -> Result<bool> {
    map(opt(tuple((tag("extend"), ignore_token1))), |extend| {
        extend.is_some()
    })(s)
}

fn definition(s: Span) -> Result<Definition> {
    alt((
        map(positioned(schema_definition), Definition::Schema),
        map(positioned(type_definition), Definition::Type),
        map(positioned(directive_definition), Definition::Directive),
    ))(s)
}

fn schema_definition(s: Span) -> Result<SchemaDefinition> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, directive_list)) = tuple((
        description(is_extend),
        tag("schema"),
        ignore_token1,
        directive_list(UNEXTEND),
    ))(s)?;
    let (s, field_list) = match opt(tuple((
        left_brace,
        ignore_token0,
        separated_list(
            ignore_token1,
            map(
                tuple((
                    alt((
                        map(tag("query"), |_| OperationType::Query),
                        map(tag("mutation"), |_| OperationType::Mutation),
                        map(tag("subscription"), |_| OperationType::Subscription),
                    )),
                    ignore_token0,
                    colon,
                    ignore_token0,
                    positioned(name),
                )),
                |(ty, _, _, _, name)| OperationField { ty, name },
            ),
        ),
        ignore_token0,
        right_brace,
    )))(s)?
    {
        (s, Some((_, _, field_list, _, _))) => (s, field_list),
        (s, None) if is_extend && directive_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        SchemaDefinition {
            is_extend,
            description,
            directive_list,
            field_list,
        },
    ))
}

fn type_definition(s: Span) -> Result<TypeDefinition> {
    alt((
        map(positioned(scalar_type), TypeDefinition::Scalar),
        map(positioned(object_type), TypeDefinition::Object),
        map(positioned(interface_type), TypeDefinition::Interface),
        map(positioned(union_type), TypeDefinition::Union),
        map(positioned(enum_type), TypeDefinition::Enum),
        map(positioned(input_object_type), TypeDefinition::InputObject),
    ))(s)
}

fn directive_definition(s: Span) -> Result<DirectiveDefinition> {
    map(
        tuple((
            description(UNEXTEND),
            tag("directive"),
            ignore_token0,
            at,
            ignore_token0,
            positioned(name),
            ignore_token0,
            field_argument_list,
            ignore_token0,
            tag("on"),
            alt((
                map(tuple((ignore_token1, pipeline, ignore_token0)), |_| ()),
                ignore_token0,
            )),
            separated_list(
                pipeline,
                map(
                    tuple((ignore_token0, positioned(directive_location), ignore_token0)),
                    |(_, directive_location, _)| directive_location,
                ),
            ),
        )),
        |(description, _, _, _, _, name, _, argument_list, _, _, _, location_list)| {
            DirectiveDefinition {
                description,
                name,
                argument_list,
                location_list,
            }
        },
    )(s)
}

fn scalar_type(s: Span) -> Result<ScalarType> {
    let (s, is_extend) = extend(s)?;

    map(
        tuple((
            description(is_extend),
            tag("scalar"),
            ignore_token1,
            positioned(name),
            directive_list(is_extend),
        )),
        move |(description, _, _, name, directive_list)| ScalarType {
            is_extend,
            description,
            name,
            directive_list,
        },
    )(s)
}

fn object_type(s: Span) -> Result<ObjectType> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, name, interface_list)) = tuple((
        description(is_extend),
        tag("type"),
        ignore_token1,
        positioned(name),
        map(
            opt(tuple((
                ignore_token0,
                tag("implements"),
                alt((
                    map(tuple((ignore_token1, ampersand, ignore_token0)), |_| ()),
                    ignore_token0,
                )),
                separated_list(
                    tuple((ignore_token0, ampersand, ignore_token0)),
                    positioned(name),
                ),
            ))),
            |interface_list| {
                interface_list
                    .map(|(_, _, _, interface_list)| interface_list)
                    .unwrap_or_else(Vec::new)
            },
        ),
    ))(s)?;
    let (s, directive_list) = directive_list(UNEXTEND)(s)?;
    let (s, field_list) = match opt(tuple((
        ignore_token0,
        left_brace,
        ignore_token0,
        field_list,
        ignore_token0,
        right_brace,
    )))(s)?
    {
        (s, Some((_, _, _, field_list, _, _))) => (s, field_list),
        (s, None) if is_extend && directive_list.is_empty() && interface_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        ObjectType {
            is_extend,
            description,
            interface_list,
            name,
            field_list,
            directive_list,
        },
    ))
}

fn interface_type(s: Span) -> Result<InterfaceType> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, name, directive_list)) = tuple((
        description(is_extend),
        tag("interface"),
        ignore_token1,
        positioned(name),
        directive_list(UNEXTEND),
    ))(s)?;
    let (s, field_list) = match opt(tuple((
        ignore_token0,
        left_brace,
        ignore_token0,
        field_list,
        ignore_token0,
        right_brace,
    )))(s)?
    {
        (s, Some((_, _, _, field_list, _, _))) => (s, field_list),
        (s, None) if is_extend && directive_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        InterfaceType {
            is_extend,
            description,
            name,
            field_list,
            directive_list,
        },
    ))
}

fn union_type(s: Span) -> Result<UnionType> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, type_name, directive_list)) = tuple((
        description(is_extend),
        tag("union"),
        ignore_token1,
        positioned(name),
        directive_list(UNEXTEND),
    ))(s)?;
    let (s, member_list) = match opt(tuple((
        ignore_token0,
        equal,
        ignore_token0,
        opt(tuple((pipeline, ignore_token0))),
        separated_list(
            tuple((ignore_token0, pipeline, ignore_token0)),
            positioned(name),
        ),
    )))(s)?
    {
        (s, Some((_, _, _, _, member_list))) => (s, member_list),
        (s, None) if is_extend && directive_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        UnionType {
            is_extend,
            description,
            name: type_name,
            member_list,
            directive_list,
        },
    ))
}

fn enum_type(s: Span) -> Result<EnumType> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, name, directive_list)) = tuple((
        description(is_extend),
        tag("enum"),
        ignore_token1,
        positioned(name),
        directive_list(UNEXTEND),
    ))(s)?;
    let (s, member_list) = match opt(tuple((
        ignore_token0,
        left_brace,
        ignore_token0,
        many1(map(
            tuple((ignore_token0, positioned(enum_member))),
            |(_, member)| member,
        )),
        ignore_token0,
        right_brace,
    )))(s)?
    {
        (s, Some((_, _, _, member_list, _, _))) => (s, member_list),
        (s, None) if is_extend && directive_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        EnumType {
            is_extend,
            description,
            name,
            member_list,
            directive_list,
        },
    ))
}

fn input_object_type(s: Span) -> Result<InputObjectType> {
    let (s, is_extend) = extend(s)?;
    let (s, (description, _, _, name, directive_list)) = tuple((
        description(is_extend),
        tag("input"),
        ignore_token1,
        positioned(name),
        directive_list(UNEXTEND),
    ))(s)?;
    let (s, field_list) = match opt(tuple((
        ignore_token0,
        left_brace,
        ignore_token0,
        field_list,
        ignore_token0,
        right_brace,
    )))(s)?
    {
        (s, Some((_, _, _, field_list, _, _))) => (s, field_list),
        (s, None) if is_extend && directive_list.is_empty() => {
            return Err(Error(ParsingError::Nom(s, ErrorKind::Char)))
        }
        (s, _) => (s, vec![]),
    };

    Ok((
        s,
        InputObjectType {
            is_extend,
            description,
            name,
            field_list,
            directive_list,
        },
    ))
}

fn enum_member(s: Span) -> Result<EnumMember> {
    map(
        tuple((
            description(UNEXTEND),
            positioned(name),
            directive_list(UNEXTEND),
        )),
        |(description, name, directive_list)| EnumMember {
            description,
            name,
            directive_list,
        },
    )(s)
}

fn field_list(s: Span) -> Result<Vec<Positioned<Field>>> {
    separated_list(ignore_token1, positioned(field))(s)
}

fn field(s: Span) -> Result<Field> {
    map(
        tuple((
            description(UNEXTEND),
            positioned(name),
            ignore_token0,
            field_argument_list,
            colon,
            ignore_token0,
            positioned(ty),
            directive_list(UNEXTEND),
        )),
        |(description, name, _, argument_list, _, _, ty, directive_list)| Field {
            description,
            name,
            argument_list,
            ty,
            directive_list,
        },
    )(s)
}

fn field_argument_list(s: Span) -> Result<Vec<Positioned<FieldArgument>>> {
    map(
        opt(tuple((
            left_parens,
            ignore_token0,
            separated_list(ignore_token1, positioned(field_argument)),
            ignore_token0,
            right_parens,
            ignore_token0,
        ))),
        |field_argument_list| {
            field_argument_list
                .map(|(_, _, field_argument, _, _, _)| field_argument)
                .unwrap_or_else(Vec::new)
        },
    )(s)
}

fn field_argument(s: Span) -> Result<FieldArgument> {
    map(
        tuple((
            description(UNEXTEND),
            positioned(name),
            ignore_token0,
            colon,
            ignore_token0,
            positioned(ty),
            opt(map(
                tuple((ignore_token0, equal, ignore_token0, positioned(value))),
                |(_, _, _, default_value)| default_value,
            )),
            directive_list(UNEXTEND),
        )),
        |(description, name, _, _, _, ty, default_value, directive_list)| FieldArgument {
            description,
            name,
            ty,
            default_value,
            directive_list,
        },
    )(s)
}

fn directive_list(is_extend: bool) -> impl Fn(Span) -> Result<Vec<Positioned<Directive>>> {
    move |s: Span| {
        if is_extend {
            many1(map(
                tuple((ignore_token0, positioned(directive))),
                |(_, directive)| directive,
            ))(s)
        } else {
            many0(map(
                tuple((ignore_token0, positioned(directive))),
                |(_, directive)| directive,
            ))(s)
        }
    }
}

fn directive(s: Span) -> Result<Directive> {
    map(
        tuple((
            at,
            ignore_token0,
            positioned(name),
            ignore_token0,
            field_argument_list,
        )),
        |(_, _, name, _, argument_list)| Directive {
            name,
            argument_list,
        },
    )(s)
}

fn ty(s: Span) -> Result<Type> {
    alt((ty_nonnull, ty_list, ty_named))(s)
}

fn ty_named(s: Span) -> Result<Type> {
    map(name, Type::Named)(s)
}

fn ty_list(s: Span) -> Result<Type> {
    map(
        tuple((
            left_bracket,
            ignore_token0,
            ty,
            ignore_token0,
            right_bracket,
        )),
        |(_, _, ty, _, _)| Type::List(Box::new(ty)),
    )(s)
}

fn ty_nonnull(s: Span) -> Result<Type> {
    map(
        tuple((alt((ty_named, ty_list)), ignore_token0, exclamation)),
        |(ty, _, _)| Type::NonNull(Box::new(ty)),
    )(s)
}

fn value(s: Span) -> Result<Value> {
    alt((
        value_null,
        value_boolean,
        value_numeric,
        value_enum,
        value_string,
        value_object,
        value_list,
    ))(s)
}

fn value_null(s: Span) -> Result<Value> {
    dbg!(s);
    map(tag("null"), |_| Value::Null)(s)
}

fn value_boolean(s: Span) -> Result<Value> {
    alt((
        map(tag("true"), |_| Value::Boolean(true)),
        map(tag("false"), |_| Value::Boolean(false)),
    ))(s)
}

fn value_numeric(s: Span) -> Result<Value> {
    enum Numeric {
        Int(usize),
        Float(usize),
    }

    let sign_to_int = |sign: Option<_>| if sign.is_some() { 1 } else { 0 };
    let len = |s: Span| s.fragment().len();
    fn zero<O, F>(f: F) -> impl Fn(Span) -> Result<O>
    where
        F: Copy + Fn(usize) -> O,
    {
        move |s: Span| map(take(0usize), |_| f(0))(s)
    }

    let (ss, sign) = map(opt(hyphen), sign_to_int)(s)?;

    let (ss, majority) = alt((
        map(tag("0"), |_| 1),
        map(
            tuple((take_while1(is_nonzero_digit), take_while(is_digit))),
            |(nonzero_digit, digit)| len(nonzero_digit) + len(digit),
        ),
    ))(ss)?;

    match alt((
        map(tuple((dot, take_while(is_digit))), move |(_, minority)| {
            if minority.fragment().is_empty() {
                Numeric::Int(sign + majority)
            } else {
                Numeric::Float(1 + len(minority))
            }
        }),
        zero(Numeric::Float),
    ))(ss)?
    {
        (_, Numeric::Int(size)) => {
            let (s, numeric) = take(size)(s)?;
            Ok((s, Value::Int(numeric.fragment().parse().unwrap())))
        }
        (ss, Numeric::Float(minority)) => {
            let (_, exponential) = alt((
                map(
                    tuple((
                        alt((char('e'), char('E'))),
                        opt(alt((plus, hyphen))),
                        take_while1(is_digit),
                    )),
                    |(_, sign, digit)| 1 + sign_to_int(sign) + len(digit),
                ),
                zero(|size| size),
            ))(ss)?;

            let (s, numeric) = take(sign + majority + minority + exponential)(s)?;

            Ok((s, Value::Float(numeric.fragment().parse().unwrap())))
        }
    }
}

fn value_enum(s: Span) -> Result<Value> {
    map(name, Value::Enum)(s)
}

fn value_string(s: Span) -> Result<Value> {
    map(string, Value::String)(s)
}

fn value_object(s: Span) -> Result<Value> {
    map(
        separated_list(
            ignore_token1,
            tuple((name, ignore_token0, colon, ignore_token0, value)),
        ),
        |pair_list| {
            let object: BTreeMap<String, Value> = pair_list
                .into_iter()
                .map(|(name, _, _, _, value)| (name, value))
                .collect();
            Value::Object(object)
        },
    )(s)
}

fn value_list(s: Span) -> Result<Value> {
    map(separated_list(ignore_token1, value), Value::List)(s)
}

fn directive_location(s: Span) -> Result<DirectiveLocation> {
    macro_rules! executable {
        ($location:expr => $variant:ident) => {
            map(tag($location), |_| {
                DirectiveLocation::Executable(ExecutableDirectiveLocation::$variant)
            })
        };
    }

    macro_rules! type_system {
        ($location:expr => $variant:ident) => {
            map(tag($location), |_| {
                DirectiveLocation::TypeSystem(TypeSystemDirectiveLocation::$variant)
            })
        };
    }

    alt((
        type_system! { "SCHEMA" => Schema },
        type_system! { "SCALAR" => Scalar },
        type_system! { "OBJECT" => Object },
        type_system! { "FIELD_DEFINITION" => FieldDefinition },
        type_system! { "ARGUMENT_DEFINITION" => ArgumentDefinition },
        type_system! { "INTERFACE" => Interface },
        type_system! { "UNION" => Union },
        type_system! { "ENUM_VALUE" => EnumValue },
        type_system! { "ENUM" => Enum },
        type_system! { "INPUT_OBJECT" => InputObject },
        type_system! { "INPUT_FIELD_DEFINITION" => InputFieldDefinition },
        executable! { "QUERY" => Query },
        executable! { "MUTATION" => Mutation },
        executable! { "SUBSCRIPTION" => Subscription },
        executable! { "FIELD" => Field },
        executable! { "FRAGMENT_DEFINITION" => FragmentDefinition },
        executable! { "FRAGMENT_SPREAD" => FragmentSpread },
        executable! { "INLINE_FRAGMENT" => InlineFragment },
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let path_list = [
            "tests/directives",
            "tests/enums",
            "tests/input_objects",
            "tests/input_values",
            "tests/interfaces",
            "tests/objects",
            "tests/scalars",
            "tests/schema",
            "tests/unions",
        ];

        for path in path_list.iter() {
            for entry in std::fs::read_dir(path).unwrap() {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let source = std::fs::read_to_string(&path).unwrap();
                    if let Err(error) = parse_schema(source.as_str()) {
                        dbg!(path);
                        dbg!(error);
                        panic!();
                    };
                }
            }
        }
    }
}
