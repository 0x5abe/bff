use std::collections::{HashMap, HashSet};

use petgraph::Graph;

use crate::names::Name;

#[derive(Default)]
pub struct DependencyIndex {
    names: HashSet<Name>,
    outgoing: HashMap<Name, Vec<Name>>,
    incoming: HashMap<Name, Vec<Name>>,
}

impl DependencyIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_node(&mut self, name: Name) {
        self.names.insert(name);
    }

    pub fn insert_edge(&mut self, from: Name, to: Name) {
        self.insert_node(from);
        self.insert_node(to);
        self.outgoing.entry(from).or_default().push(to);
        self.incoming.entry(to).or_default().push(from);
    }

    pub fn outgoing(&self, name: &Name) -> Option<&Vec<Name>> {
        self.outgoing.get(name)
    }

    pub fn incoming(&self, name: &Name) -> Option<&Vec<Name>> {
        self.incoming.get(name)
    }

    pub fn from_references<I, R>(references: I) -> Self
    where
        I: IntoIterator<Item = (Name, R)>,
        R: IntoIterator<Item = Name>,
    {
        let mut index = Self::new();

        for (from, references) in references {
            index.insert_node(from);

            for to in references {
                index.insert_edge(from, to);
            }
        }

        index
    }

    pub fn to_graph(&self) -> Graph<Name, ()> {
        let mut graph = Graph::with_capacity(self.names.len(), 0);
        let mut node_ids = HashMap::new();

        for &name in &self.names {
            node_ids.insert(name, graph.add_node(name));
        }

        for (&from, references) in &self.outgoing {
            let from_id = node_ids[&from];

            for &to in references {
                let to_id = node_ids[&to];
                graph.add_edge(from_id, to_id, ());
            }
        }

        graph
    }
}
