use petgraph::{dot::Dot, stable_graph::StableGraph, stable_graph::NodeIndex, Direction};

use std::io::prelude::*;
use std::path::Path;
use std::io::Error;
use std::fs::File;
use std::fmt;

use super::ast;

pub struct Block<'a> {
    debug_name: String,
    statements: Vec<&'a ast::nodes::Statement>,
}

impl<'a> Block<'a> {
    fn with_name(debug_name: String) -> Self {
        Block {
            debug_name,
            statements: Vec::new(),
        }
    }
}

impl<'a> fmt::Debug for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ast::visitable::Visitable;

        write!(f, "{}", self.debug_name)?;
        if self.statements.len() > 0 {
            writeln!(f, ":")?;
        }

        let mut first = true;
        for statement in self.statements.iter() {
            if first == false {
                writeln!(f)?;
            }
            first = false;

            let mut buffer = Vec::new();
            {
                let mut format = ast::format::FormatAst::with_indent(&mut buffer, 1);
                statement.visit_mut(&mut format);
            }
            let buffer = String::from_utf8_lossy(&buffer);
            write!(f, "{}", buffer)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Edge {
    Jump,
    Await,
    IfTrue,
    IfFalse,
    Break,
    Continue,
}

pub type GraphType<'a> = StableGraph<Block<'a>, Edge>;

pub struct ControlFlowGraph<'a> {
    entry_node: NodeIndex,
    exit_node: Option<NodeIndex>,
    graph: GraphType<'a>,
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

pub struct Builder<'a> {
    result: ControlFlowGraph<'a>,
    block_counter: u32,
    name: &'a str,
}

#[derive(Debug)]
struct PreviousNode {
    parent: NodeIndex,
    edge: Edge,
}

struct BuildResult {
    start_block: NodeIndex,
    end_block: NodeIndex,
    unresolved_exits: Vec<PreviousNode>,
}

impl BuildResult {
    fn resolve_exits(&mut self, kind: Edge) -> Vec<NodeIndex> {
        let mut matches = Vec::new();

        self.unresolved_exits
            .retain(|exit| {
                let mut still_unresolved = true;

                if exit.edge == kind {
                    still_unresolved = false;
                    matches.push(exit.parent);
                }

                still_unresolved
            });

        matches
    }

    fn assert_resolved(self) {
        if self.unresolved_exits.is_empty() == false {
            panic!("build result not resolved: {:#?}", self.unresolved_exits);
        }
    }
}

impl<'a> Builder<'a> {
    pub fn new(name: &'a str) -> Self {
        let mut graph = GraphType::new();
        let entry_node = graph.add_node(Block::with_name("function entry point".into()));
        Self {
            result: ControlFlowGraph {
                entry_node,
                exit_node: None,
                graph,
            },
            block_counter: 0,
            name,
        }
    }

    pub fn build(mut self, statements: &'a [ast::nodes::Statement]) -> ControlFlowGraph<'a> {
        let build_result = self.build_inner(statements);
        self.result.graph.add_edge(self.result.entry_node, build_result.start_block, Edge::Jump);
        if self.result.graph.neighbors_directed(build_result.end_block, Direction::Outgoing).count() == 0 {
            let exit_point = self.result.graph.add_node(Block::with_name("function exit point".into()));
            self.result.exit_node = Some(exit_point.clone());
            self.result.graph.add_edge(build_result.end_block, exit_point, Edge::Jump);
        }
        self.result
    }

    fn build_inner(&mut self, statements: &'a [ast::nodes::Statement]) -> BuildResult {
        let first_block = self.make_block();

        let mut result = BuildResult {
            start_block: first_block,
            end_block: first_block,
            unresolved_exits: Vec::new(),
        };

        for statement in statements.iter() {
            match statement {
                assignment @ &ast::nodes::Statement::Assignment { .. } => {
                    self.get_block_mut(result.end_block).statements.push(assignment);
                },
                fn_call @ &ast::nodes::Statement::FnCall { .. } => {
                    self.get_block_mut(result.end_block).statements.push(fn_call);
                },
                &ast::nodes::Statement::Loop(ref nested_statements) => {
                    let loop_begin = self.make_block_with_description("loop begin");
                    let loop_repeat = self.make_block_with_description("loop repeat");
                    self.result.graph.add_edge(result.end_block, loop_begin, Edge::Jump);
                    self.result.graph.add_edge(loop_repeat, loop_begin, Edge::Jump);

                    let mut nested_result = self.build_inner(nested_statements);
                    self.result.graph.add_edge(loop_begin, nested_result.start_block, Edge::Jump);
                    self.resolve_exits(&mut nested_result, Edge::Continue, loop_begin);
                    if nested_result.unresolved_exits.iter().any(|exit| exit.edge == Edge::Break) {
                        let loop_end = self.make_block_with_description("loop end");
                        self.resolve_exits(&mut nested_result, Edge::Break, loop_end);
                        result.end_block = loop_end;
                    } else {
                        result.end_block = loop_repeat;
                    }
                    self.result.graph.add_edge(nested_result.end_block, loop_repeat, Edge::Jump);
                    nested_result.assert_resolved();
                },
                await_expr @ &ast::nodes::Statement::Await(..) => {
                    self.get_block_mut(result.end_block).statements.push(await_expr);
                    let next_block = self.make_block();
                    self.result.graph.add_edge(result.end_block, next_block, Edge::Await);
                    result.end_block = next_block;
                },
            }
        }

        result
    }

    fn resolve_exits(&mut self, build_result: &mut BuildResult, kind: Edge, target: NodeIndex) {
        for node in build_result.resolve_exits(kind) {
            self.result.graph.add_edge(node, target, kind);
        }
    }

    fn make_block(&mut self) -> NodeIndex {
        let name = self.make_block_name();
        self.result.graph.add_node(Block::with_name(name))
    }

    fn make_block_with_description(&mut self, description: &str) -> NodeIndex {
        let name = format!("{} ({})", self.make_block_name(), description);
        self.result.graph.add_node(Block::with_name(name))
    }

    fn get_block_mut(&mut self, index: NodeIndex) -> &mut Block<'a> {
        self.result.graph.node_weight_mut(index).unwrap()
    }

    fn make_block_name(&mut self) -> String {
        let result = format!("{}{}", self.name, self.block_counter);
        self.block_counter += 1;
        result
    }
}
