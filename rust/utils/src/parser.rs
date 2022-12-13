use std::str::FromStr;

use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, T::from_str)(input)
}
