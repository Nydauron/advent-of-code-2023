use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;
use num::integer::lcm;

pub fn part2(input: &str) -> u64 {
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
                        state.insert(input, (Signal::Low, None));
                    }
                }
                _ => {}
            }
        }
    });

    static DESTINATION_MODULE: &str = "rx";
    let mut button_press = 0;
    loop {
        button_press += 1;
        let mut queue = VecDeque::from([SignalStep {
            source: "button", // just to match the examples, but it will never be referenced
            destination: "broadcaster",
            signal: Signal::Low,
            parent_cycle: Some(1),
        }]);
        while let Some(signal_step) = queue.pop_front() {
            // if signal_step.source == "vr" {
            //     dbg!(button_press, &signal_step);
            // }
            if signal_step.destination == DESTINATION_MODULE {
                if signal_step.parent_cycle.is_some() {
                    return signal_step.parent_cycle.unwrap();
                }
                if signal_step.signal == Signal::Low {
                    return button_press;
                }
            }
            if let Some(module) = modules.get_mut(signal_step.destination) {
                match &mut module.module_type {
                    ModuleType::Broadcaster => {
                        for &output in module.destinations.iter() {
                            queue.push_back(SignalStep {
                                source: signal_step.destination,
                                destination: output,
                                signal: Signal::Low,
                                parent_cycle: Some(1),
                            });
                        }
                    }
                    ModuleType::FlipFlop(state) => {
                        if signal_step.signal == Signal::Low {
                            *state = match state {
                                Signal::Low => Signal::High,
                                Signal::High => Signal::Low,
                            };
                            // if module.cycle.is_none() && *state == Signal::Low {
                            //     module.cycle = Some(button_press);
                            // }
                            for &output in module.destinations.iter() {
                                queue.push_back(SignalStep {
                                    source: signal_step.destination,
                                    destination: output,
                                    signal: *state,
                                    parent_cycle: module.cycle,
                                });
                            }
                        }
                    }
                    ModuleType::Conjunction(state) => {
                        state.entry(signal_step.source).and_modify(|signal_state| {
                            signal_state.0 = signal_step.signal;
                            signal_state.1 = signal_step.parent_cycle;
                        });
                        let output_signal =
                            if state.values().all(|(signal, _)| *signal == Signal::High) {
                                Signal::Low
                            } else {
                                Signal::High
                            };

                        // FIXME: I hate this
                        // This only works since the given implementation is basically 4 counters
                        // with different periods.
                        // So this cheese solution is to brute force the given period and then
                        // propagate the cycle number up to child conjuctions (since the only
                        // modules after the conjunction in the counter are conjunctions)
                        if module.cycle.is_none() {
                            if output_signal == Signal::Low && state.len() > 1 {
                                module.cycle = Some(button_press);
                            } else if state.values().all(|(_, cycle)| cycle.is_some()) {
                                module.cycle = Some(
                                    state.values().map(|(_, cycle)| cycle.unwrap()).fold(1, lcm),
                                );
                                dbg!(module.name, module.cycle.unwrap());
                            }
                        }

                        for &output in module.destinations.iter() {
                            queue.push_back(SignalStep {
                                source: signal_step.destination,
                                destination: output,
                                signal: output_signal,
                                parent_cycle: module.cycle,
                            });
                        }
                    }
                }
            }
        }

        // // check if all modules have initial state (meaning that the device has cycled once)
        // if is_at_initial_state(&modules) {
        //     println!(
        //         "Module state has returned to initial state after button press {}:",
        //         button_press
        //     );
        // }
    }
}

fn is_at_initial_state(modules: &HashMap<&str, Module>) -> bool {
    modules.values().all(|module| match &module.module_type {
        ModuleType::Broadcaster => true,
        ModuleType::FlipFlop(state) => *state == Signal::Low,
        ModuleType::Conjunction(state) => state
            .values()
            .all(|(last_input, _)| *last_input == Signal::Low),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SignalStep<'a> {
    source: &'a str,
    destination: &'a str,
    signal: Signal,
    parent_cycle: Option<u64>,
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
    Conjunction(HashMap<&'a str, (Signal, Option<u64>)>),
}
#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    name: &'a str,
    destinations: Vec<&'a str>,
    cycle: Option<u64>,
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
            cycle: None,
        },
    ))
}
