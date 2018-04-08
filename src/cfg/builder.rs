use petgraph::{Direction, stable_graph::NodeIndex};

use super::graph::{Block, Edge, GraphType};
use super::cfg::ControlFlowGraph;
use super::super::ast;

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
