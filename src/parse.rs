use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag},
    character::complete::char,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

use crate::ast::*;

fn skip_whitespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(is_a("\n\r\t "))(input)?;
    Ok((input, ()))
}

fn nil(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag("nil")(input)?;
    Ok((input, Expression::Nil))
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}

fn string(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("<<\"")(input)?;
    let (input, content) = is_not("\"")(input)?;
    let (input, _) = tag("\">>")(input)?;

    Ok((input, content.to_string()))
}

fn block(input: &str) -> IResult<&str, Block> {
    let (input, _) = char('{')(input)?;
    let (input, _) = skip_whitespace(input)?;
    let (input, name) = string(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = skip_whitespace(input)?;
    let (input, expr) = any_expr(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Block {
            key: name,
            value: expr,
        },
    ))
}

pub fn parse_document(input: &str) -> IResult<&str, Vec<Block>> {
    fn document_sep(input: &str) -> IResult<&str, ()> {
        let (input, _) = tag(".")(input)?;
        let (input, _) = skip_whitespace(input)?;
        Ok((input, ()))
    }

    let (input, _) = skip_whitespace(input)?;
    let (input, document) = separated_list0(document_sep, block)(input)?;
    let (input, _) = skip_whitespace(input)?;

    Ok((input, document))
}

fn array(input: &str) -> IResult<&str, Vec<Expression>> {
    fn array_sep(input: &str) -> IResult<&str, ()> {
        let (input, _) = tag(",")(input)?;
        let (input, _) = skip_whitespace(input)?;
        Ok((input, ()))
    }

    delimited(char('['), separated_list0(array_sep, any_expr), char(']'))(input)
}

fn hash_map(input: &str) -> IResult<&str, Vec<Block>> {
    fn key_pair(input: &str) -> IResult<&str, Block> {
        let (input, key) = string(input)?;
        let (input, _) = skip_whitespace(input)?;
        let (input, _) = tag("=>")(input)?;
        let (input, _) = skip_whitespace(input)?;
        let (input, value) = any_expr(input)?;
        Ok((input, Block { key, value }))
    }

    fn map_sep(input: &str) -> IResult<&str, ()> {
        let (input, _) = char(',')(input)?;
        let (input, _) = skip_whitespace(input)?;
        Ok((input, ()))
    }

    delimited(tag("#{"), separated_list0(map_sep, key_pair), char('}'))(input)
}

fn any_expr(input: &str) -> IResult<&str, Expression> {
    let bool_expr = map(boolean, Expression::Boolean);
    let string_expr = map(string, Expression::String);
    let array_expr = map(array, Expression::Array);
    let block_expr = map(block, |b| Expression::Block(Box::new(b)));
    let map_expr = map(hash_map, Expression::Map);

    alt((
        nil,
        bool_expr,
        string_expr,
        array_expr,
        block_expr,
        map_expr,
    ))(input)
}
