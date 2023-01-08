use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

use eyre::{bail, Context, ContextCompat, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alphanumeric1, multispace0},
        streaming::space0,
    },
    combinator::{all_consuming, map},
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

fn solve_problem(input: &str) -> Result<f64> {
    let mut program = parse_with_nom(input, parse)?;

    program.set_unknown_variable(VariableName::new("humn"));

    let (left, right) = program.get_equation(&VariableName::new("root"))?;

    let left = program.resolve(left)?;
    let right = program.resolve(right)?;

    let (unknown, literal) = match (left, right) {
        (Value::Unknown(left), Value::Literal(right)) => (left, right),
        (Value::Literal(left), Value::Unknown(right)) => (right, left),
        unsupported => unimplemented!("Solving equation {unsupported:?} is not supported"),
    };

    let result = (literal - unknown.adder) / unknown.multiplicator;
    Ok(result)
}

fn parse(input: &str) -> IResult<&str, Program> {
    all_consuming(map(
        many1(tuple((
            terminated(parse_variable_name, tuple((tag(":"), space0))),
            terminated(parse_expression, multispace0),
        ))),
        |expressions| Program {
            expressions: expressions.into_iter().collect(),
        },
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(parsing::number, |v| Expression::Value(Value::Literal(v))),
        map(parse_operation, Expression::Operation),
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
    fn resolve(&self, variable: &VariableName) -> Result<Value> {
        let variable = self.get_var(variable)?;
        let operation = match variable {
            Expression::Operation(operation) => operation,
            Expression::Value(v) => return Ok(v.clone()),
        };
        let left = self.resolve(&operation.left)?;
        let right = self.resolve(&operation.right)?;
        let result = operation.operator.exec(left, right);
        Ok(result)
    }

    fn set_unknown_variable(&mut self, name: VariableName) {
        self.expressions
            .insert(name, Expression::Value(Value::Unknown(Unknown::new())));
    }

    fn get_equation(&self, name: &VariableName) -> Result<(&VariableName, &VariableName)> {
        let expression = self.get_var(name)?;
        let Expression::Operation(operation) = expression else {
            bail!("{name:?} is not an equation")
        };
        Ok((&operation.left, &operation.right))
    }

    fn get_var(&self, name: &VariableName) -> Result<&Expression> {
        self.expressions
            .get(name)
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
    Value(Value),
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
    fn exec(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Literal(a), Value::Literal(b)) => Value::Literal(self.exec_literal(a, b)),
            (Value::Unknown(a), Value::Literal(b)) => Value::Unknown(self.exec_literal(a, b)),
            (Value::Literal(a), Value::Unknown(b)) => Value::Unknown(self.exec_literal(a, b)),
            (a, b) => unimplemented!("Cannot exec operator {self:?} on {a:?} and {b:?}"),
        }
    }

    fn exec_literal<A, B, C>(&self, a: A, b: B) -> C
    where
        A: Add<B, Output = C>
            + std::ops::Div<B, Output = C>
            + std::ops::Mul<B, Output = C>
            + std::ops::Sub<B, Output = C>,
    {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
            Operator::Div => a / b,
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Unknown(Unknown),
    Literal(N),
}

#[derive(Debug, Clone)]
struct Unknown {
    multiplicator: N,
    adder: N,
}

impl Unknown {
    fn new() -> Self {
        Self {
            multiplicator: 1.,
            adder: 0.,
        }
    }
}

impl Add<N> for Unknown {
    type Output = Unknown;

    fn add(self, n: N) -> Self::Output {
        Self {
            multiplicator: self.multiplicator,
            adder: self.adder + n,
        }
    }
}

impl Mul<N> for Unknown {
    type Output = Unknown;

    fn mul(self, n: N) -> Self::Output {
        Self {
            multiplicator: self.multiplicator * n,
            adder: self.adder * n,
        }
    }
}

impl Sub<N> for Unknown {
    type Output = Unknown;

    fn sub(self, n: N) -> Self::Output {
        self + -n
    }
}

impl Div<N> for Unknown {
    type Output = Unknown;

    fn div(self, n: N) -> Self::Output {
        self * (1. / n)
    }
}

impl Add<Unknown> for N {
    type Output = Unknown;

    fn add(self, rhs: Unknown) -> Self::Output {
        rhs + self
    }
}

impl Mul<Unknown> for N {
    type Output = Unknown;

    fn mul(self, rhs: Unknown) -> Self::Output {
        rhs * self
    }
}

impl Sub<Unknown> for N {
    type Output = Unknown;

    fn sub(self, rhs: Unknown) -> Self::Output {
        (rhs * -1.0) + self
    }
}

impl Div<Unknown> for N {
    type Output = Unknown;

    fn div(self, _rhs: Unknown) -> Self::Output {
        unimplemented!("Division by unknown not implemented");
    }
}

type N = f64;

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 301.0);
    }

    #[test]
    fn test_example2() {
        let input = read_to_string("./example2.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, -4.0);
    }
}
