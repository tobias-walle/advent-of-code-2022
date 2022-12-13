use std::collections::HashMap;
use std::mem;

use std::str::FromStr;
use std::{collections::HashSet, hash::Hash};

use eyre::{bail, Context, Result};
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::map_res,
    sequence::delimited, IResult,
};
use utils::{parser, read_input_file_as_string};

fn main() -> Result<()> {
    let input = read_input_file_as_string()?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<u32> {
    let (monkeys, mut items) = parse_input(input)?;
    let monkey_ids: Vec<_> = monkeys.iter().map(|monkey| monkey.id).collect();
    let mut monkeys: HashMap<_, _> = monkeys
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect();

    let mut inspections: HashMap<MonkeyId, u32> = HashMap::new();

    let rounds = 20;

    for _round in 0..rounds {
        for monkey_id in &monkey_ids {
            let item_ids = {
                let monkey = monkeys.get_mut(monkey_id).unwrap();
                mem::take(&mut monkey.items)
            };
            for item_id in item_ids {
                // Increment inspection counter
                inspections.insert(*monkey_id, inspections.get(monkey_id).unwrap_or(&0) + 1);

                let item = items.get_mut(&item_id).unwrap();
                item.worry_level = monkeys[monkey_id].operation.exec(item.worry_level);
                item.worry_level /= 3;

                let next_monkey_id = monkeys[monkey_id].test.next_monkey(item.worry_level);
                monkeys
                    .get_mut(&next_monkey_id)
                    .unwrap()
                    .items
                    .insert(item_id);
            }
        }
    }

    // Get top 2 number of inspections
    let mut inspections: Vec<_> = inspections.values().collect();
    inspections.sort();
    inspections.reverse();
    Ok(inspections[0] * inspections[1])
}

fn parse_input(input: &str) -> Result<(Vec<Monkey>, HashMap<ItemId, Item>)> {
    let (input, parsed) = match many1(Monkey::parse)(input) {
        Ok(parsed) => parsed,
        Err(err) => bail!("Failed to parse input: {err}"),
    };
    if !input.is_empty() {
        bail!("Couldn't parse all the input. Unparsed:\n{input}")
    }

    let items: HashMap<_, _> = parsed.iter().flat_map(|v| v.1.clone()).collect();
    let monkeys: Vec<_> = parsed.into_iter().map(|v| v.0).collect();

    Ok((monkeys, items))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MonkeyId(u32);

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    id: MonkeyId,
    items: HashSet<ItemId>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, (Monkey, HashMap<ItemId, Item>)> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Monkey ")(input)?;
        let (input, id) = map_res(digit1, u32::from_str)(input)?;
        let (input, _) = tag(":")(input)?;
        let id = MonkeyId(id);

        let (input, items) = Item::parse_starting_items(id, input)?;
        let (input, computation) = Operation::parse(input)?;
        let (input, test) = Test::parse(input)?;

        let (input, _) = multispace0(input)?;

        let monkey = Monkey {
            id,
            items: items.keys().cloned().collect(),
            operation: computation,
            test,
        };

        Ok((input, (monkey, items)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Operation {
    left: Variable,
    operator: Operator,
    right: Variable,
}

impl Operation {
    fn exec(&self, old: u128) -> u128 {
        let left = self.left.resolve(old);
        let right = self.right.resolve(old);
        self.operator.apply(left, right)
    }
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Operation: new = ")(input)?;
        let (input, (left, operator, right)) =
            tuple((Variable::parse, Operator::parse, Variable::parse))(input)?;
        Ok((
            input,
            Operation {
                left,
                operator,
                right,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Variable {
    Literal(u128),
    Old,
}

impl Variable {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        alt((
            map(tag("old"), |_| Self::Old),
            map(parser::number, Self::Literal),
        ))(input)
    }

    fn resolve(&self, old: u128) -> u128 {
        match self {
            Self::Old => old,
            Self::Literal(n) => *n,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        alt((
            map(tag("*"), |_| Self::Multiply),
            map(tag("+"), |_| Self::Add),
        ))(input)
    }

    fn apply(&self, a: u128, b: u128) -> u128 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ItemId(u32, u32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    id: ItemId,
    worry_level: u128,
}

impl Item {
    fn parse_starting_items(monkey: MonkeyId, input: &str) -> IResult<&str, HashMap<ItemId, Item>> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Starting items:")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, worry_levels) = separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parser::number,
        )(input)?;
        let items: HashMap<_, _> = worry_levels
            .into_iter()
            .enumerate()
            .map(|(i, worry_level)| Item {
                id: ItemId(monkey.0, i as u32),
                worry_level,
            })
            .map(|item| (item.id, item))
            .collect();

        Ok((input, items))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Test {
    divisible_by: u128,
    if_true_throw_to: MonkeyId,
    if_false_throw_to: MonkeyId,
}

impl Test {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("Test: divisible by")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, divisible_by) = parser::number(input)?;

        let (input, _) = multispace0(input)?;
        let (input, _) = tag("If true: throw to monkey")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, if_true_throw_to) = map(parser::number, MonkeyId)(input)?;

        let (input, _) = multispace0(input)?;
        let (input, _) = tag("If false: throw to monkey")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, if_false_throw_to) = map(parser::number, MonkeyId)(input)?;

        Ok((
            input,
            Test {
                divisible_by,
                if_true_throw_to,
                if_false_throw_to,
            },
        ))
    }

    fn next_monkey(&self, worry_level: u128) -> MonkeyId {
        if worry_level % self.divisible_by == 0 {
            self.if_true_throw_to
        } else {
            self.if_false_throw_to
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_monkey() {
        let (_, (monkey, items)) = Monkey::parse(
            "
            Monkey 0:
            Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            "
            .trim(),
        )
        .unwrap();

        let expected_monkey = Monkey {
            id: MonkeyId(0),
            items: HashSet::from([ItemId(0, 0), ItemId(0, 1)]),
            test: Test {
                divisible_by: 23,
                if_true_throw_to: MonkeyId(2),
                if_false_throw_to: MonkeyId(3),
            },
            operation: Operation {
                left: Variable::Old,
                operator: Operator::Multiply,
                right: Variable::Literal(19),
            },
        };

        let expected_items = HashMap::from([
            (
                ItemId(0, 0),
                Item {
                    id: ItemId(0, 0),
                    worry_level: 79,
                },
            ),
            (
                ItemId(0, 1),
                Item {
                    id: ItemId(0, 1),
                    worry_level: 98,
                },
            ),
        ]);

        assert_eq!(expected_monkey, monkey);
        assert_eq!(expected_items, items);
    }
}
