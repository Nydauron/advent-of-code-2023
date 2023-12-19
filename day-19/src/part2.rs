use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::{is_not, tag, take_until1},
    character::complete::{self, one_of},
    sequence::delimited,
    IResult,
};

pub fn part2(input: &str) -> u64 {
    let (rules, _) = input
        .split_once("\n\n")
        .expect("Input did not have double newlines");

    let rules = rules
        .lines()
        .map(|line| {
            let (_, rule) = parse_rule(line).expect("Line did not parse correctly");
            (rule.name, rule)
        })
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::from([(
        "in",
        PartRange {
            x_cool: (1..=4000),
            musical: (1..=4000),
            aerodynamic: (1..=4000),
            shiny: (1..=4000),
        },
    )]);
    let mut accepted_part_ranges: Vec<PartRange> = vec![];

    while let Some((rule_name, mut property_ranges)) = queue.pop_front() {
        let curr = rules
            .get(rule_name)
            .expect("Rule name was not found in rule map");
        let accepted = curr.cases.iter().filter_map(|case| {
            if let Some(conditional) = case.conditional {
                match conditional {
                    Condition::GreaterThan(property_condition) => match property_condition {
                        Property::XCool(boundary) => {
                            if *property_ranges.x_cool.end() <= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    *property_ranges.x_cool.start()..=boundary,
                                    (boundary + 1)..=*property_ranges.x_cool.end(),
                                );

                                property_ranges.x_cool = leftover;
                                let new_part_range = PartRange {
                                    x_cool: new_bounds,
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Musical(boundary) => {
                            if *property_ranges.musical.end() <= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    *property_ranges.musical.start()..=boundary,
                                    (boundary + 1)..=*property_ranges.musical.end(),
                                );

                                property_ranges.musical = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: new_bounds,
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Aerodynamic(boundary) => {
                            if *property_ranges.aerodynamic.end() <= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    *property_ranges.aerodynamic.start()..=boundary,
                                    (boundary + 1)..=*property_ranges.aerodynamic.end(),
                                );

                                property_ranges.aerodynamic = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: new_bounds,
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Shiny(boundary) => {
                            if *property_ranges.shiny.end() <= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    *property_ranges.shiny.start()..=boundary,
                                    (boundary + 1)..=*property_ranges.shiny.end(),
                                );

                                property_ranges.shiny = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: new_bounds,
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                    },
                    Condition::LessThan(property_condition) => match property_condition {
                        Property::XCool(boundary) => {
                            if *property_ranges.x_cool.start() >= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    boundary..=*property_ranges.x_cool.end(),
                                    *property_ranges.x_cool.start()..=(boundary - 1),
                                );

                                property_ranges.x_cool = leftover;
                                let new_part_range = PartRange {
                                    x_cool: new_bounds,
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Musical(boundary) => {
                            if *property_ranges.musical.start() >= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    boundary..=*property_ranges.musical.end(),
                                    *property_ranges.musical.start()..=(boundary - 1),
                                );

                                property_ranges.musical = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: new_bounds,
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Aerodynamic(boundary) => {
                            if *property_ranges.aerodynamic.start() >= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    boundary..=*property_ranges.aerodynamic.end(),
                                    *property_ranges.aerodynamic.start()..=(boundary - 1),
                                );

                                property_ranges.aerodynamic = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: new_bounds,
                                    shiny: property_ranges.shiny.clone(),
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                        Property::Shiny(boundary) => {
                            if *property_ranges.shiny.start() >= boundary {
                                None
                            } else {
                                let (leftover, new_bounds) = (
                                    boundary..=*property_ranges.shiny.end(),
                                    *property_ranges.shiny.start()..=(boundary - 1),
                                );

                                property_ranges.shiny = leftover;
                                let new_part_range = PartRange {
                                    x_cool: property_ranges.x_cool.clone(),
                                    musical: property_ranges.musical.clone(),
                                    aerodynamic: property_ranges.aerodynamic.clone(),
                                    shiny: new_bounds,
                                };
                                match case.operation {
                                    Operation::Goto(next_rule) => {
                                        queue.push_back((next_rule, new_part_range));
                                        None
                                    }
                                    Operation::Accept => Some(new_part_range),
                                    Operation::Reject => None,
                                }
                            }
                        }
                    },
                }
            } else {
                match case.operation {
                    Operation::Goto(next_rule) => {
                        queue.push_back((next_rule, property_ranges.clone()));
                        None
                    }
                    Operation::Accept => Some(property_ranges.clone()),
                    Operation::Reject => None,
                }
            }
        });

        accepted_part_ranges.extend(accepted);
    }

    accepted_part_ranges
        .iter()
        .map(|part_range| {
            (part_range.x_cool.end() - part_range.x_cool.start() + 1) as u64
                * (part_range.musical.end() - part_range.musical.start() + 1) as u64
                * (part_range.aerodynamic.end() - part_range.aerodynamic.start() + 1) as u64
                * (part_range.shiny.end() - part_range.shiny.start() + 1) as u64
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    x_cool: RangeInclusive<u32>,
    musical: RangeInclusive<u32>,
    aerodynamic: RangeInclusive<u32>,
    shiny: RangeInclusive<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Property {
    XCool(u32),
    Musical(u32),
    Aerodynamic(u32),
    Shiny(u32),
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

fn parse_property(property: &str, amount: u32) -> Result<Property, &'static str> {
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
    let (input, property_amount) = complete::u32(input)?;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
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

        assert_eq!(part2(input), 167409079868000);
    }
}
