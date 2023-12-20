use std::collections::{BTreeMap, VecDeque};
use std::hash::Hash;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map, success},
    multi::separated_list1,
    sequence::terminated,
    Finish as _, IResult,
};

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let others = lcm(&nums[1..]);
        nums[0] * others / gcd(nums[0], others)
    }
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Signal {
    Lo,
    Hi,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Module<'a> {
    FlipFlop(FlipFlopState),
    Conjunction(BTreeMap<&'a str, Signal>),
    Broadcaster,
    Sink,
}
impl<'a> Module<'a> {
    fn register_connection(&mut self, name: &'a str) {
        if let Module::Conjunction(ref mut last_seen_states) = self {
            last_seen_states.insert(name, Signal::Lo);
        }
    }

    fn receive_signal(&mut self, sender: &'a str, signal: Signal) -> Option<Signal> {
        match self {
            Module::FlipFlop(ref mut state) => match signal {
                Signal::Lo => match state {
                    FlipFlopState::On => {
                        *state = FlipFlopState::Off;
                        Some(Signal::Lo)
                    }
                    FlipFlopState::Off => {
                        *state = FlipFlopState::On;
                        Some(Signal::Hi)
                    }
                },
                Signal::Hi => None,
            },
            Module::Conjunction(ref mut last_seen_states) => {
                *last_seen_states.get_mut(sender).unwrap() = signal;

                if last_seen_states.values().all(|v| matches!(v, Signal::Hi)) {
                    Some(Signal::Lo)
                } else {
                    Some(Signal::Hi)
                }
            }
            Module::Broadcaster => Some(signal),
            Module::Sink => None,
        }
    }
}

// returns (module, name, connections)
fn parse_node<'a>(input: &'a str) -> IResult<&'a str, (Module<'a>, &'a str, Vec<&'a str>)> {
    let (input, module) = alt((
        map(tag("%"), |_| Module::FlipFlop(FlipFlopState::Off)),
        map(tag("&"), |_| Module::Conjunction(BTreeMap::default())),
        map(success(()), |_| Module::Broadcaster),
    ))(input)?;
    let (input, name) = terminated(alpha1, tag(" -> "))(input)?;
    let (input, connections) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((input, (module, name, connections)))
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct ModuleNetwork<'a> {
    node_connections: BTreeMap<&'a str, Vec<&'a str>>,
    nodes: BTreeMap<&'a str, Module<'a>>,
}
impl<'a> ModuleNetwork<'a> {
    // returns (num lo, num hi) signals sent as a result of pressing the button
    fn send_signal(
        &mut self,
        target: &'a str,
        signal: Signal,
        observe_from: &'a str,
        observe_to: &'a str,
    ) -> (u64, u64, bool) {
        let mut num_lo = 0;
        let mut num_hi = 0;
        let mut observed = false;

        let mut signal_queue = VecDeque::new();
        signal_queue.push_front(("button", target, signal));

        while let Some((sender, target, signal)) = signal_queue.pop_front() {
            // println!("sending {signal:?} from {sender} to {target}");
            match signal {
                Signal::Lo => {
                    num_lo += 1;
                }
                Signal::Hi => {
                    num_hi += 1;
                }
            }

            if sender == observe_from && target == observe_to && signal == Signal::Hi {
                observed = true;
            }

            if let Some(new_signal) = self
                .nodes
                .get_mut(target)
                .unwrap()
                .receive_signal(sender, signal)
            {
                for new_target in self.node_connections.get(target).unwrap() {
                    signal_queue.push_back((target, new_target, new_signal));
                }
            }
        }

        (num_lo, num_hi, observed)
    }

    fn button_presses_until_signal(&self, final_conjunction: &'a str, observ_prev: &'a str) -> u64 {
        let mut count = 0;

        let mut working_state = self.clone();

        loop {
            count += 1;

            let (_, _, observed) = working_state.send_signal(
                "broadcaster",
                Signal::Lo,
                observ_prev,
                final_conjunction,
            );
            if observed {
                return count;
            }
        }
    }

    fn parse(input: &'a str) -> IResult<&'a str, ModuleNetwork<'a>> {
        let mut network = ModuleNetwork::default();

        for line in input.lines() {
            let (_, (module, name, connections)) = parse_node(line).finish().unwrap();

            network.nodes.insert(name, module);
            network.node_connections.insert(name, connections);
        }

        // register all connections
        for (node_name, connections) in &network.node_connections {
            for connection in connections {
                network
                    .nodes
                    .entry(connection)
                    .or_insert(Module::Sink)
                    .register_connection(&node_name);
            }
        }

        Ok((input, network))
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (_, network) = ModuleNetwork::parse(&input).unwrap();

    // p1
    {
        let mut network = network.clone();

        let mut lo_count = 0;
        let mut hi_count = 0;

        for _ in 0..1000 {
            let (addtl_lo, addtl_hi, _) = network.send_signal("broadcaster", Signal::Lo, "", "");
            lo_count += addtl_lo;
            hi_count += addtl_hi;
        }
        // dbg!(&network);

        println!("{}", lo_count * hi_count);
    }

    // p2
    {
        // pm, mk, pk, hf -> vf -> rx
        let pm_cycle = dbg!(network.button_presses_until_signal("vf", "pm"));
        let mk_cycle = dbg!(network.button_presses_until_signal("vf", "mk"));
        let pk_cycle = dbg!(network.button_presses_until_signal("vf", "pk"));
        let hf_cycle = dbg!(network.button_presses_until_signal("vf", "hf"));

        println!("{}", lcm(&[pm_cycle, mk_cycle, pk_cycle, hf_cycle]));
        println!("{}", pm_cycle * mk_cycle * pk_cycle * hf_cycle);
    }
}
