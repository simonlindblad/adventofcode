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

struct BitSet(u64);

impl BitSet {
    fn add(&mut self, handle: NodeHandle) {
        self.0 |= 1 << handle
    }

    fn contains(&self, handle: NodeHandle) -> bool {
        self.0 & (1 << handle) > 0
    }

    fn remove(&mut self, handle: NodeHandle) {
        self.0 ^= 1 << handle
    }

    fn value(&self) -> u64 {
        self.0
    }
}

fn dfs(
    tunnels: &TunnelSystem,
    current: NodeHandle,
    remaining: i32,
    visited: &mut BitSet,
    dp: &mut HashMap<(NodeHandle, u64, i32), i32>,
) -> i32 {
    if remaining <= 0 {
        return 0;
    } else if dp.contains_key(&(current, visited.value(), remaining)) {
        return dp[&(current, visited.value(), remaining)];
    }

    let valve = tunnels.get(current);
    let mut local_max = if valve.flow > 0 && !visited.contains(current) {
        visited.add(current);
        let local_max =
            valve.flow * (remaining - 1) + dfs(tunnels, current, remaining - 1, visited, dp);
        visited.remove(current);
        local_max
    } else {
        0
    };

    for child in valve.children.iter() {
        local_max = max(local_max, dfs(tunnels, *child, remaining - 1, visited, dp));
    }

    dp.insert((current, visited.value(), remaining), local_max);
    local_max
}

/// This is nasty - but didn't have time to clean it up
fn dfs2(
    tunnels: &TunnelSystem,
    you: NodeHandle,
    ele: NodeHandle,
    remaining: i32,
    visited: &mut BitSet,
    dp: &mut HashMap<(NodeHandle, NodeHandle, u64, i32), i32>,
) -> i32 {
    if remaining <= 0 {
        return 0;
    } else if dp.contains_key(&(you, ele, visited.value(), remaining)) {
        return dp[&(you, ele, visited.value(), remaining)];
    } else if dp.contains_key(&(ele, you, visited.value(), remaining)) {
        return dp[&(ele, you, visited.value(), remaining)];
    }

    let your_valve = tunnels.get(you);
    let ele_valve = tunnels.get(ele);

    // Both open
    let mut local_max = if you != ele
        && your_valve.flow > 0
        && ele_valve.flow > 0
        && !visited.contains(you)
        && !visited.contains(ele)
    {
        visited.add(you);
        visited.add(ele);
        let local_max = ele_valve.flow * (remaining - 1)
            + your_valve.flow * (remaining - 1)
            + dfs2(tunnels, you, ele, remaining - 1, visited, dp);
        visited.remove(you);
        visited.remove(ele);
        local_max
    } else {
        0
    };

    let you_open = if your_valve.flow > 0 && !visited.contains(you) {
        let mut you_open = 0;
        visited.add(you);
        for child in ele_valve.children.iter() {
            you_open = max(
                you_open,
                your_valve.flow * (remaining - 1)
                    + dfs2(tunnels, you, *child, remaining - 1, visited, dp),
            )
        }
        visited.remove(you);
        you_open
    } else {
        0
    };

    local_max = max(local_max, you_open);

    let ele_open = if ele_valve.flow > 0 && !visited.contains(ele) {
        let mut ele_open = 0;
        visited.add(ele);
        for child in your_valve.children.iter() {
            ele_open = max(
                ele_open,
                ele_valve.flow * (remaining - 1)
                    + dfs2(tunnels, *child, ele, remaining - 1, visited, dp),
            )
        }
        visited.remove(ele);
        ele_open
    } else {
        0
    };
    local_max = max(local_max, ele_open);

    // both
    let mut both = 0;
    for ychild in your_valve.children.iter() {
        for echild in ele_valve.children.iter() {
            both = max(
                both,
                dfs2(tunnels, *ychild, *echild, remaining - 1, visited, dp),
            );
        }
    }

    local_max = max(local_max, both);
    dp.insert((you, ele, visited.value(), remaining), local_max);
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
        dfs(&tunnels, start, 30, &mut BitSet(0), &mut HashMap::new())
    );
    // Did this with finding all possible combination of disjoint set at time 26...
    // but this v1 solution also did the trick after 4 mins.
    println!(
        "Part 2: {}",
        dfs2(
            &tunnels,
            start,
            start,
            26,
            &mut BitSet(0),
            &mut HashMap::new()
        )
    )
}
