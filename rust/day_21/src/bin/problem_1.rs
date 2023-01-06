use std::collections::HashMap;

use eyre::{Context, ContextCompat, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alphanumeric1, multispace0},
        streaming::space0,
    },
    combinator::map,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use utils::{
    parsing::{self, parse_with_nom},
    read_input_file_as_string,
};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<N> {
    let program = parse_with_nom(input, parse)?;
    let result = program.resolve(&VariableName::new("root"))?;
    Ok(result)
}

fn parse(input: &str) -> IResult<&str, Program> {
    map(
        many1(tuple((
            terminated(parse_variable_name, tuple((tag(":"), space0))),
            terminated(parse_expression, multispace0),
        ))),
        |expressions| Program {
            expressions: expressions.into_iter().collect(),
        },
    )(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parsing::number, |v| Expression::Literal(v)),
        map(parse_operation, |v| Expression::Operation(v)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    map(
        tuple((
            parse_variable_name,
            delimited(space0, parse_operator, space0),
            parse_variable_name,
        )),
        |(left, operator, right)| Operation {
            left,
            operator,
            right,
        },
    )(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("+"), |_| Operator::Add),
        map(tag("-"), |_| Operator::Sub),
        map(tag("*"), |_| Operator::Mul),
        map(tag("/"), |_| Operator::Div),
    ))(input)
}

fn parse_variable_name(input: &str) -> IResult<&str, VariableName> {
    map(alphanumeric1, VariableName::new)(input)
}

#[derive(Debug, Clone)]
struct Program {
    expressions: HashMap<VariableName, Expression>,
}

impl Program {
    fn resolve(&self, variable: &VariableName) -> Result<N> {
        let variable = self.get_var(variable)?;
        let operation = match variable {
            Expression::Operation(operation) => operation,
            Expression::Literal(n) => return Ok(*n),
        };
        let left = self.resolve(&operation.left)?;
        let right = self.resolve(&operation.right)?;
        let result = operation.operator.exec(left, right);
        Ok(result)
    }

    fn get_var(&self, name: &VariableName) -> Result<&Expression> {
        self.expressions
            .get(&name)
            .with_context(|| format!("{name:?} not found"))
    }
}

#[derive(Debug, Clone)]
struct Operation {
    left: VariableName,
    operator: Operator,
    right: VariableName,
}

#[derive(Debug, Clone)]
enum Expression {
    Operation(Operation),
    Literal(N),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct VariableName(String);

impl VariableName {
    fn new(name: &str) -> Self {
        Self(name.into())
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn exec(&self, a: N, b: N) -> N {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
            Operator::Div => a / b,
        }
    }
}

type N = i64;

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 152);
    }
}
