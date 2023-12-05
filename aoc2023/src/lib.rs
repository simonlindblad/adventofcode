use std::{env, path::Path};

fn read_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn read_input_lines() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    read_file(&args[1])
}

pub fn read_input_content() -> String {
    let args: Vec<String> = env::args().collect();
    std::fs::read_to_string(&args[1]).expect("Something went wrong reading the file")
}

/// Macro to construct a HashMap
#[macro_export]
macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut t = ::std::collections::HashMap::new();
         $( t.insert($key, $val); )*
         t
    }}
}
