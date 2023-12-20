use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;

pub fn part1(input: &str) -> u64 {
    let mut modules = input
        .lines()
        .map(|line| {
            let (_, module) = parse_module(line).expect("Module line could not be parsed");
            (module.name, module)
        })
        .collect::<HashMap<_, _>>();

    let inputs_to_modules = modules
        .values()
        .flat_map(|module| {
            module
                .destinations
                .iter()
                .map(|dest_module| (dest_module, module.name))
        })
        .fold(HashMap::new(), |mut acc, (&module, input)| {
            acc.entry(module)
                .and_modify(|inputs: &mut HashSet<_>| {
                    inputs.insert(input);
                })
                .or_insert(HashSet::from([input]));
            acc
        });

    modules.iter_mut().for_each(|(key, module)| {
        if let Some(inputs) = inputs_to_modules.get(key) {
            match &mut module.module_type {
                ModuleType::Conjunction(state) => {
                    for &input in inputs {
                        state.insert(input, Signal::Low);
                    }
                }
                _ => {}
            }
        }
    });

    const TOTAL_BUTTON_PRESSES: usize = 1000;
    let mut total_low_signals = 0;
    let mut total_high_signals = 0;
    for _ in 0..TOTAL_BUTTON_PRESSES {
        let mut queue = VecDeque::from([SignalStep {
            source: "button", // just to match the examples, but it will never be referenced
            destination: "broadcaster",
            signal: Signal::Low,
        }]);
        while let Some(signal_step) = queue.pop_front() {
            match signal_step.signal {
                Signal::Low => total_low_signals += 1,
                Signal::High => total_high_signals += 1,
            };
            if let Some(module) = modules.get_mut(signal_step.destination) {
                // .expect(format!("Recieved non-existant module: {:?}", signal_step).as_str());

                match &mut module.module_type {
                    ModuleType::Broadcaster => {
                        for &output in module.destinations.iter() {
                            queue.push_back(SignalStep {
                                source: signal_step.destination,
                                destination: output,
                                signal: Signal::Low,
                            });
                        }
                    }
                    ModuleType::FlipFlop(state) => {
                        if signal_step.signal == Signal::Low {
                            *state = match state {
                                Signal::Low => Signal::High,
                                Signal::High => Signal::Low,
                            };
                            for &output in module.destinations.iter() {
                                queue.push_back(SignalStep {
                                    source: signal_step.destination,
                                    destination: output,
                                    signal: *state,
                                });
                            }
                        }
                    }
                    ModuleType::Conjunction(state) => {
                        state.entry(signal_step.source).and_modify(|signal_state| {
                            *signal_state = signal_step.signal;
                        });
                        let output_signal = if state.values().all(|signal| *signal == Signal::High)
                        {
                            Signal::Low
                        } else {
                            Signal::High
                        };
                        for &output in module.destinations.iter() {
                            queue.push_back(SignalStep {
                                source: signal_step.destination,
                                destination: output,
                                signal: output_signal,
                            });
                        }
                    }
                }
            }
        }
    }

    total_low_signals * total_high_signals
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SignalStep<'a> {
    source: &'a str,
    destination: &'a str,
    signal: Signal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop(Signal),
    Conjunction(HashMap<&'a str, Signal>),
}
#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    name: &'a str,
    destinations: Vec<&'a str>,
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, (module_type, name)) = alt((
        map(tag("%"), |_| (ModuleType::FlipFlop(Signal::Low), "")),
        map(tag("&"), |_| (ModuleType::Conjunction(HashMap::new()), "")),
        map(tag("broadcaster"), |name| (ModuleType::Broadcaster, name)),
    ))(input)?;
    let (input, name) = if module_type != ModuleType::Broadcaster {
        alpha1(input)?
    } else {
        (input, name)
    };

    let (input, _) = tag(" -> ")(input)?;
    let destinations = input
        .split(",")
        .map(|module_dest| module_dest.trim())
        .collect_vec();
    Ok((
        "",
        Module {
            module_type,
            name,
            destinations,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_simple() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_part1_complex() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(part1(input), 11687500);
    }
}
