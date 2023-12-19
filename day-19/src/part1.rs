use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag, take_until1},
    character::complete::{self, one_of},
    sequence::delimited,
    IResult,
};

pub fn part1(input: &str) -> u64 {
    let (rules, parts) = input
        .split_once("\n\n")
        .expect("Input did not have double newlines");

    let rules = rules
        .lines()
        .map(|line| {
            let (_, rule) = parse_rule(line).expect("Line did not parse correctly");
            (rule.name, rule)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line| {
            let (_, part) = parse_part(line).expect("Could not parse part line");
            part
        })
        .collect_vec();

    parts
        .iter()
        .filter(|part| {
            let mut curr = rules.get("in").expect("no start node");

            loop {
                let satisfied_case = curr
                    .cases
                    .iter()
                    .find(|case| {
                        if let Some(conditional) = case.conditional {
                            match conditional {
                                Condition::GreaterThan(property_condition) => {
                                    match property_condition {
                                        Property::XCool(condition_amount) => {
                                            part.x_cool > condition_amount
                                        }
                                        Property::Musical(condition_amount) => {
                                            part.musical > condition_amount
                                        }
                                        Property::Aerodynamic(condition_amount) => {
                                            part.aerodynamic > condition_amount
                                        }
                                        Property::Shiny(condition_amount) => {
                                            part.shiny > condition_amount
                                        }
                                    }
                                }
                                Condition::LessThan(property_condition) => match property_condition
                                {
                                    Property::XCool(condition_amount) => {
                                        part.x_cool < condition_amount
                                    }
                                    Property::Musical(condition_amount) => {
                                        part.musical < condition_amount
                                    }
                                    Property::Aerodynamic(condition_amount) => {
                                        part.aerodynamic < condition_amount
                                    }
                                    Property::Shiny(condition_amount) => {
                                        part.shiny < condition_amount
                                    }
                                },
                            }
                        } else {
                            true
                        }
                    })
                    .expect("case not found");
                match satisfied_case.operation {
                    Operation::Goto(next_rule) => {
                        curr = rules.get(next_rule).expect("no start node")
                    }
                    Operation::Accept => return true,
                    Operation::Reject => return false,
                }
            }
        })
        .fold(0, |acc, accepted_part| {
            acc + accepted_part.x_cool
                + accepted_part.musical
                + accepted_part.aerodynamic
                + accepted_part.shiny
        })
}

struct Part {
    x_cool: u64,
    musical: u64,
    aerodynamic: u64,
    shiny: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    XCool(u64),
    Musical(u64),
    Aerodynamic(u64),
    Shiny(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    GreaterThan(Property),
    LessThan(Property),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation<'a> {
    Goto(&'a str),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Case<'a> {
    conditional: Option<Condition>,
    operation: Operation<'a>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule<'a> {
    name: &'a str,
    cases: Vec<Case<'a>>,
}

fn parse_property(property: &str, amount: u64) -> Result<Property, &'static str> {
    match property {
        "x" => Ok(Property::XCool(amount)),
        "m" => Ok(Property::Musical(amount)),
        "a" => Ok(Property::Aerodynamic(amount)),
        "s" => Ok(Property::Shiny(amount)),
        _ => Err("Not a valid property"),
    }
}

fn parse_conditional(input: &str) -> IResult<&str, Condition> {
    let (input, property_initial) = is_not("<>")(input)?;
    let (input, inequality) = one_of("<>")(input)?;
    let (input, property_amount) = complete::u64(input)?;
    let property =
        parse_property(property_initial, property_amount).expect("property was not valid");
    match inequality {
        '>' => Ok((input, Condition::GreaterThan(property))),
        '<' => Ok((input, Condition::LessThan(property))),
        _ => panic!("idk how to create nom error lmao"),
    }
}

fn parse_operation(input: &str) -> Operation {
    let operation = match input {
        "A" => Operation::Accept,
        "R" => Operation::Reject,
        name => Operation::Goto(name),
    };
    operation
}

fn parse_case(input: &str) -> IResult<&str, Case> {
    if let Some((conditional, operation)) = input.split_once(':') {
        let (_, conditional) = parse_conditional(conditional)?;
        let operation = parse_operation(operation);
        Ok((
            "",
            Case {
                conditional: Some(conditional),
                operation,
            },
        ))
    } else {
        let operation = parse_operation(input);
        Ok((
            "",
            Case {
                conditional: None,
                operation,
            },
        ))
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = take_until1("{")(input)?;
    let (input, rules_str) = delimited(tag("{"), is_not("}"), tag("}"))(input)?;
    let mut cases = vec![];
    for case in rules_str.split(',') {
        let (_, case) = parse_case(case)?;
        cases.push(case);
    }

    Ok((input, Rule { name, cases }))
}

fn parse_property_assignment(input: &str) -> IResult<&str, Property> {
    let (input, property_initial) = is_not("=")(input)?;
    let (input, _) = one_of("=")(input)?;
    let (input, property_amount) = complete::u64(input)?;
    let property =
        parse_property(property_initial, property_amount).expect("Property could not be parsed");
    Ok((input, property))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, property_str) = delimited(tag("{"), is_not("}"), tag("}"))(input)?;
    let part = property_str
        .split(',')
        .map(|property| {
            let (_, property) =
                parse_property_assignment(property).expect("Property could not be parsed");
            property
        })
        .fold(
            Part {
                x_cool: 0,
                musical: 0,
                aerodynamic: 0,
                shiny: 0,
            },
            |mut acc, property| {
                match property {
                    Property::XCool(amount) => acc.x_cool = amount,
                    Property::Musical(amount) => acc.musical = amount,
                    Property::Aerodynamic(amount) => acc.aerodynamic = amount,
                    Property::Shiny(amount) => acc.shiny = amount,
                }
                acc
            },
        );
    Ok((input, part))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(part1(input), 19114);
    }
}
