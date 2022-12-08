use std::fs::read_to_string;

type NodeHandle = usize;

#[derive(Debug)]
struct FileSystemNode {
    name: String,
    size: Option<u32>,
    children: Vec<NodeHandle>,
    parent: Option<NodeHandle>
}

impl FileSystemNode {
    fn new_dir(name: String, parent: Option<NodeHandle>) -> Self {
        FileSystemNode { name, size: None, children: Vec::new(), parent }
    }
    
    fn new_file(name: String, size: u32, parent: NodeHandle) -> Self {
        FileSystemNode { name, size: Some(size), children: Vec::new(), parent: Some(parent) }
    }

    fn is_dir(&self) -> bool {
        self.size.is_none()
    }
}

#[derive(Default)]
struct FileSystem {
    nodes: Vec<FileSystemNode>,
}

impl FileSystem {
    fn get(&self, handle: NodeHandle) -> &FileSystemNode {
        self.nodes.get(handle).expect("No such node")
    }

    fn get_mut(&mut self, handle: NodeHandle) -> &mut FileSystemNode {
        self.nodes.get_mut(handle).expect("No such node")
    }

    fn get_child(&self, handle: NodeHandle, name: &str) -> Option<NodeHandle> {
        self.get(handle).children
            .iter()
            .map(|handle| (*handle, self.get(*handle)))
            .find(|(_, node)| node.name == name)
            .map(|(handle, _)| handle)
    }

    fn add_node(&mut self, node: FileSystemNode) -> NodeHandle {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn size_of(&self, handle: NodeHandle) -> u32 {
        // Ideally we could use memoization here... but no need to optimize unless we need it.
        let node = self.get(handle);
        if let Some(size) = node.size {
            size
        } else {
            node.children.iter()
                .map(|handle| self.size_of(*handle))
                .sum()
        }
    }

    fn iter(&self) -> impl Iterator<Item = NodeHandle> {
        0..self.nodes.len()
    }
}

fn parse_filesystem(file: &str) -> FileSystem {
    let mut fs = FileSystem::default();
    let root_handle = fs.add_node(FileSystemNode::new_dir("/".to_string(), None));
    let mut cwd = root_handle;


    for line in read_to_string(file).unwrap().lines() {
        if line == "$ cd /" {
            cwd = root_handle
        } else if line == "$ cd .." {
            cwd = fs.get(cwd).parent.expect("No parent")
        } else if line.starts_with("$ cd ") {
            let name = line.strip_prefix("$ cd ").unwrap();
            if let Some(child) = fs.get_child(cwd, name) {
                cwd = child
            } else {
                panic!("Not found");
            }
        } else if line.starts_with("dir ") {
            let name = line.strip_prefix("dir ").unwrap().to_string();
            let new_dir = fs.add_node(FileSystemNode::new_dir(name, Some(cwd)));
            fs.get_mut(cwd).children.push(new_dir);
        } else if !line.starts_with("$ ls") {
            let (size, name) = line.split_once(' ').unwrap();
            let new_file = fs.add_node(FileSystemNode::new_file(name.to_string(), size.parse().unwrap(), cwd));
            fs.get_mut(cwd).children.push(new_file);
        }
    }

    fs
}

fn part1(file: &str) -> u32 {
    let limit = 100000;
    let fs = parse_filesystem(file);
    fs.iter()
        .filter(|handle| fs.get(*handle).is_dir())
        .map(|handle| fs.size_of(handle))
        .filter(|size| size <= &limit)
        .sum()
}

fn part2(file: &str) -> u32 {
    let total_size = 40000000;
    let fs = parse_filesystem(file);
    let used = fs.size_of(0);
    fs.iter()
        .filter(|handle| fs.get(*handle).is_dir())
        .map(|handle| fs.size_of(handle))
        .filter(|size| used - size <= total_size)
        .min()
        .unwrap()
}

pub fn run(file: &str) {
    println!("Part 1: {}", part1(file));
    println!("Part 2: {}", part2(file));
}
