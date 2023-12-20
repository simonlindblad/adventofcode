use std::collections::{HashMap, VecDeque};

use aoc2023::read_input_content;

#[derive(Debug, Clone)]
struct ObjectRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Default for ObjectRange {
    fn default() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

impl ObjectRange {
    fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1
    }

    fn combinations(&self) -> u64 {
        (self.x.1 as u64 - self.x.0 as u64 + 1)
            * (self.m.1 as u64 - self.m.0 as u64 + 1)
            * (self.a.1 as u64 - self.a.0 as u64 + 1)
            * (self.s.1 as u64 - self.s.0 as u64 + 1)
    }
}

#[derive(Default, Debug)]
struct Object {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Object {
    fn parse(line: &str) -> Self {
        let components = line
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',');
        let mut obj = Self::default();
        for component in components {
            let (var, val) = component.split_once('=').unwrap();
            match (var, val.parse::<u32>().unwrap()) {
                ("x", x) => obj.x = x,
                ("m", m) => obj.m = m,
                ("a", a) => obj.a = a,
                ("s", s) => obj.s = s,
                _ => unreachable!(),
            }
        }

        obj
    }
}

#[derive(Debug)]
struct Rule {
    condition_var: char,
    condition_operator: char,
    condition_target: u32,
    destination: String,
}

impl Rule {
    fn parse(input: &str) -> Rule {
        let condition_var = input.chars().nth(0).unwrap();
        let condition_operator = input.chars().nth(1).unwrap();
        let tmp = input.chars().skip(2).collect::<String>();

        let (condition_target, destination) = tmp.split_once(':').unwrap();

        Self {
            condition_var,
            condition_operator,
            condition_target: condition_target.parse::<u32>().unwrap(),
            destination: destination.into(),
        }
    }

    fn evaluate(&self, obj: &Object) -> Option<String> {
        let value = match self.condition_var {
            'a' => obj.a,
            'm' => obj.m,
            'x' => obj.x,
            's' => obj.s,
            _ => unreachable!(),
        };

        let matching = match self.condition_operator {
            '>' => value > self.condition_target,
            '<' => value < self.condition_target,
            _ => unreachable!(),
        };

        if matching {
            Some(self.destination.clone())
        } else {
            None
        }
    }

    fn inside_range(&self, obj: &ObjectRange) -> Option<ObjectRange> {
        let mut new = obj.clone();
        match (self.condition_var, self.condition_operator) {
            ('a', '>') => new.a.0 = self.condition_target + 1,
            ('a', '<') => new.a.1 = self.condition_target - 1,
            ('m', '>') => new.m.0 = self.condition_target + 1,
            ('m', '<') => new.m.1 = self.condition_target - 1,
            ('x', '>') => new.x.0 = self.condition_target + 1,
            ('x', '<') => new.x.1 = self.condition_target - 1,
            ('s', '>') => new.s.0 = self.condition_target + 1,
            ('s', '<') => new.s.1 = self.condition_target - 1,
            _ => unreachable!(),
        }

        if new.is_valid() {
            Some(new)
        } else {
            None
        }
    }

    fn outside_range(&self, obj: &ObjectRange) -> Option<ObjectRange> {
        let mut new = obj.clone();
        match (self.condition_var, self.condition_operator) {
            ('a', '>') => new.a.1 = self.condition_target,
            ('a', '<') => new.a.0 = self.condition_target,
            ('m', '>') => new.m.1 = self.condition_target,
            ('m', '<') => new.m.0 = self.condition_target,
            ('x', '>') => new.x.1 = self.condition_target,
            ('x', '<') => new.x.0 = self.condition_target,
            ('s', '>') => new.s.1 = self.condition_target,
            ('s', '<') => new.s.0 = self.condition_target,
            _ => unreachable!(),
        }

        if new.is_valid() {
            Some(new)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: String,
}

impl Workflow {
    fn parse(line: &str) -> Workflow {
        let line = line.strip_suffix('}').unwrap();
        let (name, rules) = line.split_once('{').unwrap();
        let rules = rules.split(',').collect::<Vec<_>>();
        let parsed = rules
            .iter()
            .take(rules.len() - 1)
            .map(|r| Rule::parse(r))
            .collect::<Vec<_>>();

        Workflow {
            name: name.into(),
            rules: parsed,
            fallback: (*rules.last().unwrap()).into(),
        }
    }

    fn evaluate(&self, object: &Object) -> String {
        for rule in self.rules.iter() {
            if let Some(dst) = rule.evaluate(object) {
                return dst;
            }
        }

        self.fallback.clone()
    }

    fn evaluate_range(&self, mut range: ObjectRange) -> Vec<(String, ObjectRange)> {
        let mut mapping = Vec::new();
        for rule in self.rules.iter() {
            if let Some(dst) = rule.inside_range(&range) {
                mapping.push((rule.destination.clone(), dst));
            }

            if let Some(rem) = rule.outside_range(&range) {
                range = rem;
            }
        }

        if range.is_valid() {
            mapping.push((self.fallback.clone(), range));
        }

        mapping
    }
}

fn run_workflows(workflows: &HashMap<String, Workflow>, obj: &Object) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let next = workflow.evaluate(obj);
        if next == "A" {
            return true;
        } else if next == "R" {
            return false;
        }

        workflow = workflows.get(&next).unwrap();
    }
}

fn search_range(workflows: &HashMap<String, Workflow>) -> Vec<ObjectRange> {
    let mut queue = VecDeque::new();
    queue.push_back(("in".into(), ObjectRange::default()));

    let mut accepted = Vec::new();

    while let Some((name, obj)) = queue.pop_front() {
        let workflow = workflows.get(&name).unwrap();
        for range in workflow.evaluate_range(obj) {
            if range.0 == "A" {
                accepted.push(range.1);
            } else if range.0 != "R" {
                queue.push_back((range.0, range.1));
            }
        }
    }

    accepted
}

fn main() {
    let content = read_input_content();

    let (workflows, objects) = content.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(Workflow::parse)
        .map(|w| (w.name.clone(), w))
        .collect::<HashMap<_, _>>();
    let objects = objects.lines().map(Object::parse).collect::<Vec<_>>();

    let accepted = objects
        .iter()
        .filter(|o| run_workflows(&workflows, o))
        .map(|o| o.m + o.s + o.x + o.a)
        .sum::<u32>();
    println!("Part 1: {:?}", accepted);

    let accepted_ranges = search_range(&workflows);
    let accepted = accepted_ranges
        .iter()
        .map(|o| o.combinations())
        .sum::<u64>();
    println!("Part 2: {:?}", accepted);
}
