use std::{fmt::Debug, str::FromStr};

use eyre::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), T::from_str)(input)
}

pub fn parse_with_nom<P, T>(input: &str, parse: P) -> Result<T>
where
    P: FnOnce(&str) -> IResult<&str, T>,
    T: Debug,
{
    let (_, parsed) = match parse(input) {
        Ok(parsed) => parsed,
        Err(err) => {
            bail!("Failed to parse input: {err}")
        }
    };
    Ok(parsed)
}
