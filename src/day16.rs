use std::collections::HashMap;
use std::fs::read_to_string;

use regex::Regex;

use crate::max;

type NodeHandle = usize;

#[derive(Debug)]
struct Valve {
    name: String,
    flow: i32,
    children: Vec<NodeHandle>,
}

impl Valve {
    fn with_name(name: String) -> Valve {
        Valve {
            name,
            flow: 0,
            children: Vec::new(),
        }
    }
}

#[derive(Default)]
struct TunnelSystem {
    valves: Vec<Valve>,
    handle_map: HashMap<String, NodeHandle>,
}

impl TunnelSystem {
    fn get(&self, handle: NodeHandle) -> &Valve {
        self.valves.get(handle).expect("No such valve")
    }

    fn get_mut(&mut self, handle: NodeHandle) -> &mut Valve {
        self.valves.get_mut(handle).expect("No such valve")
    }

    fn add_valve(&mut self, valve: Valve) -> NodeHandle {
        let index = self.valves.len();
        self.handle_map.insert(valve.name.clone(), index);
        self.valves.push(valve);
        index
    }

    fn get_handle(&self, name: &str) -> NodeHandle {
        *self.handle_map.get(name).unwrap()
    }

    fn get_handle_or_insert(&mut self, name: &str, valve: Valve) -> NodeHandle {
        if let Some(handle) = self.handle_map.get(name) {
            *handle
        } else {
            self.add_valve(valve)
        }
    }
}

fn dfs(
    tunnels: &TunnelSystem,
    current: NodeHandle,
    remaining: i32,
    mut visited: u64,
    dp: &mut HashMap<(NodeHandle, u64, i32), i32>,
) -> i32 {
    if remaining <= 0 {
        return 0;
    } else if dp.contains_key(&(current, visited, remaining)) {
        return dp[&(current, visited, remaining)];
    }

    let valve = tunnels.get(current);
    let mut local_max = 0;
    for child in valve.children.iter() {
        let without_open = dfs(tunnels, *child, remaining - 1, visited, dp);

        let with_open = if valve.flow > 0 && (visited & (1 << current)) == 0 {
            visited |= 1 << current;
            let with_open = valve.flow * (remaining - 1) as i32
                + dfs(tunnels, *child, remaining - 2, visited, dp);
            visited ^= 1 << current;
            with_open
        } else {
            0
        };

        local_max = max(local_max, max(without_open, with_open))
    }

    dp.insert((current, visited, remaining), local_max);
    local_max
}

fn parse_input(file: &str) -> TunnelSystem {
    let mut tunnels = TunnelSystem::default();

    let re = Regex::new(
        r"Valve (..) has flow rate=(-?\d+); tunnels? leads? to valves? ((?:[A-Z]{2},? ?)*)",
    )
    .unwrap();
    for cap in re.captures_iter(&read_to_string(file).unwrap()) {
        let name = &cap[1];
        let handle = tunnels.get_handle_or_insert(name, Valve::with_name(name.to_string()));
        let children = cap[3]
            .split(", ")
            .map(|c| tunnels.get_handle_or_insert(c, Valve::with_name(c.to_string())))
            .collect::<Vec<_>>();

        let mut valve = tunnels.get_mut(handle);
        valve.flow = cap[2].parse::<i32>().unwrap();
        valve.children = children;
    }

    tunnels
}

pub fn run(file: &str) {
    let tunnels = parse_input(file);
    let start = tunnels.get_handle("AA");
    println!(
        "Part 1: {}",
        dfs(&tunnels, start, 30, 0, &mut HashMap::new())
    )
}
