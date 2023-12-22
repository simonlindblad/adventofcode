use std::collections::{HashMap, VecDeque};

use aoc2023::read_input_lines;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "high"),
            Pulse::Low => write!(f, "low"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Node {
    Broadcaster,
    Button,
    FlipFlop(bool),
    Conjunction,
    Output,
}

type NodeIndex = usize;

struct Graph {
    nodes: Vec<Node>,
    name_to_index: HashMap<String, NodeIndex>,
    edges: HashMap<NodeIndex, Vec<NodeIndex>>,
    inputs: HashMap<NodeIndex, Vec<NodeIndex>>,
    last_sent: Vec<Option<Pulse>>,
}

impl Graph {
    fn send(&mut self, index: NodeIndex, pulse: Pulse) -> (Pulse, Vec<NodeIndex>) {
        let output_pulse = match self.nodes[index] {
            Node::Broadcaster => pulse,
            Node::Button => Pulse::Low,
            Node::FlipFlop(on) => {
                if pulse == Pulse::High {
                    // No output
                    return (Pulse::Low, Vec::new());
                } else if on {
                    self.nodes[index] = Node::FlipFlop(false);
                    Pulse::Low
                } else {
                    self.nodes[index] = Node::FlipFlop(true);
                    Pulse::High
                }
            }
            Node::Conjunction => {
                let mut output_pulse = Pulse::Low;
                for input in &self.inputs[&index] {
                    if self.last_sent[*input].unwrap_or(Pulse::Low) == Pulse::Low {
                        output_pulse = Pulse::High;
                        break;
                    }
                }
                output_pulse
            }
            Node::Output => {
                self.last_sent[index] = Some(pulse);
                return (pulse, Vec::new());
            }
        };

        self.last_sent[index] = Some(output_pulse);
        (output_pulse, self.edges[&index].clone())
    }

    fn press_button(&mut self) -> (usize, usize) {
        let (mut low_pulses, mut high_pulses) = (0, 0);
        let mut queue = VecDeque::from(vec![(0, Pulse::Low, self.name_to_index["button"])]);

        while let Some((origin, pulse, index)) = queue.pop_front() {
            if pulse == Pulse::High && index == 15 {
                println!("Sent high to {} from {}", index, origin);
                println!("Last sent: {:?}", self.last_sent[origin]);
            }
            let (output_pulse, outputs) = self.send(index, pulse);
            match output_pulse {
                Pulse::High => high_pulses += outputs.len(),
                Pulse::Low => low_pulses += outputs.len(),
            };

            queue.extend(outputs.into_iter().map(|i| (index, output_pulse, i)));
        }

        (low_pulses, high_pulses)
    }

    fn press_button_until_rx(&mut self) -> u64 {
        // We know that there's only one parent from the graphviz... "hp" that's a conjuction,
        // and we'll just need to detect when all of its inputs are high.
        let rx_parent = self.inputs[&self.name_to_index["rx"]][0];
        let mut rx_parent_cycles = HashMap::<usize, u64>::new();
        let mut presses = 0;
        while rx_parent_cycles.len() != self.inputs[&rx_parent].len() {
            presses += 1;
            let mut queue = VecDeque::from(vec![(0, Pulse::Low, self.name_to_index["button"])]);

            while let Some((origin, pulse, index)) = queue.pop_front() {
                if pulse == Pulse::High && index == rx_parent {
                    rx_parent_cycles.insert(origin, presses);
                }
                let (output_pulse, outputs) = self.send(index, pulse);
                queue.extend(outputs.into_iter().map(|i| (index, output_pulse, i)));
            }
        }

        // We have all the cycles, now we find the least common multiple that should be when
        // all of them are high
        rx_parent_cycles
            .values()
            .copied()
            .reduce(aoc2023::lcm)
            .unwrap()
    }
}

fn parse() -> Graph {
    let mut nodes = Vec::new();
    let mut name_to_index = HashMap::new();
    let mut edges = HashMap::<NodeIndex, Vec<NodeIndex>>::new();
    let mut inputs = HashMap::<NodeIndex, Vec<NodeIndex>>::new();

    let mut targets = Vec::<Vec<String>>::new();
    for line in read_input_lines() {
        let components = line.split(" -> ").collect::<Vec<_>>();
        let (node, name) = if let Some(name) = components[0].strip_prefix('%') {
            (Node::FlipFlop(false), name)
        } else if let Some(name) = components[0].strip_prefix('&') {
            (Node::Conjunction, name)
        } else if components[0] == "broadcaster" {
            (Node::Broadcaster, "broadcaster")
        } else {
            panic!("Unknown node type: {}", components[0]);
        };

        nodes.push(node);
        let index = nodes.len() - 1;
        name_to_index.insert(name.into(), index);

        targets.push(components[1].split(", ").map(|s| s.to_string()).collect());
    }

    nodes.push(Node::Button);
    let button_index = nodes.len() - 1;
    name_to_index.insert("button".into(), button_index);
    targets.push(vec!["broadcaster".into()]);

    for (index, target) in targets.iter().enumerate() {
        for name in target {
            if !name_to_index.contains_key(name) {
                nodes.push(Node::Output);
                let output_index = nodes.len() - 1;
                name_to_index.insert(name.into(), output_index);
            }

            let target_index = name_to_index[name];
            edges.entry(index).or_default().push(target_index);
            inputs.entry(target_index).or_default().push(index);
        }
    }

    let last_sent = vec![None; nodes.len()];
    Graph {
        nodes,
        name_to_index,
        edges,
        inputs,
        last_sent,
    }
}

fn part1() {
    let mut graph = parse();
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let (low_res, high_res) = graph.press_button();
        low += low_res;
        high += high_res;
    }
    println!("Part 1: {}", low * high);

    let mut graph = parse();
    let total_presses = graph.press_button_until_rx();
    println!("Part 2: {}", total_presses);
}

fn main() {
    part1();
}
