use petgraph::{dot::Dot, stable_graph::NodeIndex, Direction};

use std::io::prelude::*;
use std::path::Path;
use std::io::Error;
use std::fs::File;

use super::graph::GraphType;

pub struct ControlFlowGraph<'a> {
    pub entry_node: NodeIndex,
    pub exit_node: Option<NodeIndex>,
    pub graph: GraphType<'a>,
}

impl<'a> ControlFlowGraph<'a> {
    pub fn to_dotfile<P: AsRef<Path>>(&self, filename: P) -> Result<(), Error> {
        let contents = format!("{:#?}", Dot::new(&self.graph));

        let mut file = File::create(filename)?;
        file.write_all(contents.as_bytes())
    }

    pub fn tidy_graph(&mut self) {
        self.graph.retain_nodes(|g, idx| {
            g.neighbors_undirected(idx).count() > 0
        });

        loop {
            let mut removed_node = false;

            let indicies = self.graph.node_indices().collect::<Vec<_>>();
            for idx in indicies {
                let empty = self.graph[idx].statements.is_empty();

                if empty {
                    let incoming = self.graph.neighbors_directed(idx, Direction::Incoming)
                        .collect::<Vec<_>>();
                    let outgoing = self.graph.neighbors_directed(idx, Direction::Outgoing)
                        .collect::<Vec<_>>();

                    if incoming.len() > 0 && outgoing.len() == 1 {
                        let destination_idx = outgoing[0];
                        for source_idx in incoming {
                            let edge_idx = self.graph.find_edge(source_idx, idx).unwrap();
                            let kind = self.graph[edge_idx];
                            self.graph.add_edge(source_idx, destination_idx, kind);
                        }
                        self.graph.remove_node(idx);
                        removed_node = true;
                        break;
                    }
                }
            }

            if removed_node == false {
                break;
            }
        }
    }
}
