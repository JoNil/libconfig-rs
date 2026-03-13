use crate::{ArrayType, Value};
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while},
    character::{
        complete::{alpha1, char, one_of},
        streaming::alphanumeric1,
    },
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::{many0_count, many1, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
mod string;

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    alt((
        value(true, tag_no_case("true")),
        value(false, tag_no_case("false")),
    ))(input)
}

fn number<'a, E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>>(
    i: &'a str,
) -> IResult<&'a str, i64, E> {
    terminated(
        alt((
            map_res(
                recognize(many1(one_of("0123456789.eE"))),
                |digit_str: &str| digit_str.parse::<i64>(),
            ),
            map_res(
                preceded(tag("-"), recognize(many1(one_of("0123456789.eE")))),
                |digit_str: &str| digit_str.parse::<i64>().map(|v| -v),
            ),
        )),
        opt(tag("L")),
    )(i)
}

fn key<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "ident",
        recognize(pair(
            alt((alpha1, tag("*"))),
            many0_count(alt((alphanumeric1, tag("_"), tag("-"), tag("*")))),
        )),
    )(i)
}

fn string<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, String, E> {
    context("string", string::parse)(i)
}

fn array<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<Value>, E> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), libconfig_value),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn list<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Vec<Value>, E> {
    context(
        "list",
        preceded(
            char('('),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), libconfig_value),
                preceded(sp, char(')')),
            )),
        ),
    )(i)
}

fn key_value<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, Value), E> {
    terminated(
        separated_pair(
            preceded(sp, key),
            cut(preceded(sp, one_of("=:"))),
            libconfig_value,
        ),
        tag(";"),
    )(i)
}

fn hash<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, IndexMap<String, Value>, E> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(separated_list0(sp, key_value), |tuple_vec| {
                    tuple_vec
                        .into_iter()
                        .map(|(k, v)| (String::from(k), v))
                        .collect()
                }),
                preceded(sp, char('}')),
            )),
        ),
    )(i)
}

fn libconfig_value<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Value, E> {
    preceded(
        sp,
        alt((
            map(hash, Value::Object),
            map(array, |v| Value::Array(v, ArrayType::Array)),
            map(list, |v| Value::Array(v, ArrayType::List)),
            map(string, Value::String),
            map(boolean, Value::Bool),
            map(number, Value::Int),
            map(double, Value::Float),
        )),
    )(i)
}

pub fn root<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Value, E> {
    delimited(sp, map(key_value, |(_, v)| v), opt(sp))(i)
}
