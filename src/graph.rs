use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub struct SiteGraph {
    map: Mutex<HashMap<String, HashSet<String>>>,
}

impl SiteGraph {
    pub fn new() -> Self {
        SiteGraph {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_link(&self, from: &str, to: &str) {
        let mut graph = self.map.lock().unwrap();
        graph.entry(from.to_string()).or_default().insert(to.to_string());
    }
}

pub fn write_dot(graph: &SiteGraph, path: &str) -> std::io::Result<()> {
    let g = graph.map.lock().unwrap();
    let mut file = File::create(path)?;
    writeln!(file, "digraph site {{")?;
    for (from, tos) in g.iter() {
        for to in tos {
            writeln!(file, "\"{}\" -> \"{}\";", from, to)?;
        }
    }
    writeln!(file, "}}")?;
    Ok(())
}